import binascii
import time
from base64 import b64decode
from typing import TYPE_CHECKING, Any, Literal, NotRequired, TypedDict
from uuid import UUID

import google.cloud.pubsub_v1 as pubsub
import msgspec
from google.cloud.pubsub_v1.publisher.exceptions import MessageTooLargeError
from opentelemetry import trace
from opentelemetry.trace.propagation.tracecontext import TraceContextTextMapPropagator
from packages.db.src.enums import (
    SourceIndexingStatusEnum,
)
from packages.shared_utils.src.env import get_env
from packages.shared_utils.src.exceptions import (
    BackendError,
    DeserializationError,
    ValidationError,
)
from packages.shared_utils.src.logger import get_logger
from packages.shared_utils.src.otel import get_tracer
from packages.shared_utils.src.ref import Ref
from packages.shared_utils.src.serialization import deserialize, serialize
from packages.shared_utils.src.shared_types import EntityType
from packages.shared_utils.src.sync import run_sync

if TYPE_CHECKING:
    from google.cloud.pubsub_v1.types import PubsubMessage

logger = get_logger(__name__)

client_ref = Ref[pubsub.PublisherClient]()
subscriber_client_ref = Ref[pubsub.SubscriberClient]()
subscriber_paths: set[str] = set()


def inject_trace_context(attributes: dict[str, str] | None = None) -> dict[str, str]:
    if attributes is None:
        attributes = {}

    propagator = TraceContextTextMapPropagator()
    propagator.inject(attributes)

    return attributes


def extract_trace_context(message: "PubsubMessage") -> Any:
    attributes = dict(message.attributes) if message.attributes else {}

    propagator = TraceContextTextMapPropagator()
    return propagator.extract(attributes)


def create_pubsub_publish_span(topic_name: str, message_type: str | None = None) -> trace.Span:
    tracer = get_tracer("pubsub.publisher")

    span = tracer.start_span("pubsub.publish")
    span.set_attribute("messaging.system", "pubsub")
    span.set_attribute("messaging.destination", topic_name)
    span.set_attribute("messaging.operation", "publish")

    if message_type:
        span.set_attribute("messaging.message.type", message_type)

    return span


def create_pubsub_receive_span(
    subscription_name: str,
    message: "PubsubMessage",
    context: Any | None = None,
) -> trace.Span:
    tracer = get_tracer("pubsub.subscriber")

    span = tracer.start_span("pubsub.receive", context=context)
    span.set_attribute("messaging.system", "pubsub")
    span.set_attribute("messaging.source", subscription_name)
    span.set_attribute("messaging.operation", "receive")
    span.set_attribute("messaging.message.id", message.message_id)

    if message.attributes and (trace_id := message.attributes.get("trace_id")):
        span.set_attribute("trace_id", trace_id)

    return span


class PubSubMessage(msgspec.Struct, rename="camel"):
    publish_time: str | None = None
    message_id: str | None = None
    data: str | None = None
    attributes: dict[str, str] | None = None
    ordering_key: str | None = None


class PubSubEvent(msgspec.Struct, rename="camel"):
    message: PubSubMessage
    subscription: str | None = None


class CrawlingRequest(TypedDict):
    source_id: UUID
    entity_type: EntityType
    entity_id: UUID
    url: str
    trace_id: str


class SourceProcessingResult(TypedDict):
    source_id: UUID
    indexing_status: SourceIndexingStatusEnum
    identifier: str
    trace_id: str


class BaseRagRequest(msgspec.Struct):
    trace_id: str


class GrantApplicationRagRequest(BaseRagRequest, tag="grant_application"):
    parent_id: UUID


class GrantTemplateRagRequest(BaseRagRequest, tag="grant_template"):
    parent_id: UUID


class ResearchPlanAutofillRequest(BaseRagRequest, tag="research_plan_autofill"):
    application_id: UUID
    field_name: str | None = None
    context: dict[str, Any] | None = None


class ResearchDeepDiveAutofillRequest(BaseRagRequest, tag="research_deep_dive_autofill"):
    application_id: UUID
    field_name: str | None = None
    context: dict[str, Any] | None = None


RagRequest = (
    GrantApplicationRagRequest | GrantTemplateRagRequest | ResearchPlanAutofillRequest | ResearchDeepDiveAutofillRequest
)


class WebsocketMessage[T](TypedDict):
    type: Literal["info", "error", "warning", "success", "data"]
    parent_id: UUID
    event: str
    data: T
    trace_id: str
    application_data: NotRequired[dict[str, Any]]


class EmailNotificationRequest(TypedDict):
    application_id: UUID
    trace_id: str


class SubscriptionVerificationRequest(TypedDict):
    email: str
    subscription_id: str
    verification_token: str
    template_type: Literal["subscription_verification"]
    search_params: NotRequired[dict[str, Any]]
    frequency: NotRequired[str]
    trace_id: str


def get_publisher_client() -> pubsub.PublisherClient:
    if not client_ref.value:
        client_ref.value = pubsub.PublisherClient()

    return client_ref.value


def get_subscriber_client() -> pubsub.SubscriberClient:
    if not subscriber_client_ref.value:
        client = pubsub.SubscriberClient()
        subscriber_client_ref.value = client

    return subscriber_client_ref.value


def decode_pubsub_message(event: PubSubEvent) -> str:
    logger.debug(
        "Decoding PubSub event",
        message_id=event.message.message_id,
        publish_time=event.message.publish_time,
    )

    try:
        encoded_data = event.message.data
        if not encoded_data:
            logger.error("PubSub message missing data field", message_id=event.message.message_id)
            raise ValidationError("PubSub message missing data field")

        logger.debug("Decoding base64 data", data_length=len(encoded_data))
        return b64decode(encoded_data).decode()
    except (binascii.Error, UnicodeDecodeError) as e:
        logger.error(
            "Failed to parse PubSub message",
            error=str(e),
            message_id=event.message.message_id,
            error_type=type(e).__name__,
        )
        raise ValidationError("Invalid pubsub message format", context={"error": str(e)}) from e


async def publish_url_crawling_task(
    *,
    url: str,
    source_id: str | UUID,
    entity_type: EntityType,
    entity_id: str | UUID,
    trace_id: str,
) -> str:
    client = get_publisher_client()

    data = CrawlingRequest(
        url=url,
        source_id=UUID(str(source_id)),
        entity_type=entity_type,
        entity_id=UUID(str(entity_id)),
        trace_id=trace_id,
    )

    try:
        message_data = serialize(data)
        topic_path = client.topic_path(
            project=get_env("GCP_PROJECT_ID", fallback="grantflow"),
            topic=get_env("URL_CRAWLING_PUBSUB_TOPIC", fallback="url-crawling"),
        )

        with create_pubsub_publish_span(topic_path, "CrawlingRequest") as span:
            span.set_attribute("url", url)
            span.set_attribute("source_id", str(source_id))
            if trace_id:
                span.set_attribute("trace_id", trace_id)

            attributes = {"trace_id": trace_id} if trace_id else {}
            attributes = inject_trace_context(attributes)

            future = client.publish(topic_path, message_data, **attributes)
            message_id = await run_sync(future.result)

            span.set_attribute("messaging.message.id", message_id)

        logger.info(
            "Published message to crawl URL",
            message_id=message_id,
            url=url,
            source_id=str(source_id),
            trace_id=trace_id,
        )
        return str(message_id)
    except MessageTooLargeError as e:
        logger.error("Error publishing URL crawling message", error=str(e))
        raise BackendError("Error publishing URL crawling message", context={"error": str(e)}) from e


async def publish_rag_task(
    *,
    parent_type: Literal["grant_application", "grant_template"],
    parent_id: str | UUID,
    trace_id: str,
) -> str:
    start_time = time.time()
    logger.debug(
        "Starting PubSub message publishing",
        parent_type=parent_type,
        parent_id=str(parent_id),
        trace_id=trace_id,
    )

    client = get_publisher_client()

    data: GrantApplicationRagRequest | GrantTemplateRagRequest
    if parent_type == "grant_application":
        data = GrantApplicationRagRequest(
            parent_id=UUID(str(parent_id)),
            trace_id=trace_id,
        )
    else:
        data = GrantTemplateRagRequest(
            parent_id=UUID(str(parent_id)),
            trace_id=trace_id,
        )

    try:
        message_data = serialize(data)
        logger.debug(
            "Serialized RAG request data",
            parent_type=parent_type,
            parent_id=str(parent_id),
            message_size=len(message_data),
        )

        topic_path = client.topic_path(
            project=get_env("GCP_PROJECT_ID", fallback="grantflow"),
            topic=get_env("RAG_PROCESSING_PUBSUB_TOPIC", fallback="rag-processing"),
        )

        logger.debug(
            "Publishing message to PubSub topic",
            parent_type=parent_type,
            parent_id=str(parent_id),
            topic_path=topic_path,
        )

        with create_pubsub_publish_span(topic_path, "RagRequest") as span:
            span.set_attribute("parent_type", parent_type)
            span.set_attribute("parent_id", str(parent_id))
            if trace_id:
                span.set_attribute("trace_id", trace_id)

            attributes = {"trace_id": trace_id} if trace_id else {}
            attributes = inject_trace_context(attributes)

            future = client.publish(topic_path, message_data, **attributes)
            message_id = await run_sync(future.result)

            span.set_attribute("messaging.message.id", message_id)

        publish_duration = time.time() - start_time
        logger.info(
            "Published message to process RAG",
            message_id=message_id,
            parent_type=parent_type,
            parent_id=str(parent_id),
            trace_id=trace_id,
            publish_duration_ms=round(publish_duration * 1000, 2),
        )
        return str(message_id)
    except MessageTooLargeError as e:
        logger.error("Error publishing RAG processing message", error=str(e))
        raise BackendError("Error publishing RAG processing message", context={"error": str(e)}) from e


async def publish_autofill_task(
    *,
    parent_id: str | UUID,
    autofill_type: Literal["research_plan", "research_deep_dive"],
    field_name: str | None = None,
    context: dict[str, Any] | None = None,
    trace_id: str,
) -> str:
    start_time = time.time()

    client = get_publisher_client()

    autofill_request: ResearchPlanAutofillRequest | ResearchDeepDiveAutofillRequest
    if autofill_type == "research_plan":
        autofill_request = ResearchPlanAutofillRequest(
            application_id=UUID(str(parent_id)),
            trace_id=trace_id,
            field_name=field_name,
            context=context,
        )
    else:
        autofill_request = ResearchDeepDiveAutofillRequest(
            application_id=UUID(str(parent_id)),
            trace_id=trace_id,
            field_name=field_name,
            context=context,
        )

    try:
        message_data = serialize(autofill_request)

        logger.debug(
            "Publishing autofill task",
            parent_id=str(autofill_request.application_id),
            request_type=type(autofill_request).__name__,
            field_name=autofill_request.field_name,
            trace_id=trace_id,
            message_size=len(message_data),
        )

        topic_path = client.topic_path(
            project=get_env("GCP_PROJECT_ID", fallback="grantflow"),
            topic=get_env("RAG_PROCESSING_PUBSUB_TOPIC", fallback="rag-processing"),
        )

        with create_pubsub_publish_span(topic_path, "AutofillRequest") as span:
            span.set_attribute("parent_id", str(parent_id))
            span.set_attribute("autofill_type", autofill_type)
            if field_name:
                span.set_attribute("field_name", field_name)
            if trace_id:
                span.set_attribute("trace_id", trace_id)

            attributes = {"trace_id": trace_id} if trace_id else {}
            attributes = inject_trace_context(attributes)

            future = client.publish(topic_path, message_data, **attributes)
            message_id = await run_sync(future.result)

            span.set_attribute("messaging.message.id", message_id)

        publish_duration = time.time() - start_time
        logger.info(
            "Published autofill task",
            message_id=message_id,
            parent_id=str(parent_id),
            autofill_type=autofill_type,
            field_name=field_name,
            trace_id=trace_id,
            publish_duration_ms=round(publish_duration * 1000, 2),
        )
        return str(message_id)
    except MessageTooLargeError as e:
        logger.error("Error publishing autofill task", error=str(e))
        raise BackendError("Error publishing autofill task", context={"error": str(e)}) from e


async def ensure_subscription_for_parent_id(parent_id: UUID) -> str:
    subscriber = get_subscriber_client()
    project_id = get_env("GCP_PROJECT_ID", fallback="grantflow")
    topic_id = get_env("FRONTEND_NOTIFICATIONS_PUBSUB_TOPIC", fallback="frontend-notifications")
    topic_path = subscriber.topic_path(project=project_id, topic=topic_id)

    subscription_id = f"frontend-notifications-sub-{parent_id}"
    subscription_path = str(subscriber.subscription_path(project=project_id, subscription=subscription_id))

    if subscription_path in subscriber_paths:
        logger.debug("Subscription path already exists", subscription_path=subscription_path)
        return subscription_path

    logger.debug(
        "Ensuring subscription for parent ID",
        parent_id=str(parent_id),
        subscription_path=subscription_path,
    )

    try:
        await run_sync(
            subscriber.create_subscription,
            request={
                "name": subscription_path,
                "topic": topic_path,
                "filter": f'attributes.parent_id = "{parent_id}"',
                "ack_deadline_seconds": 600,
            },
        )
        logger.info("subscription created", subscription_path=subscription_path)
    except Exception as e:
        logger.debug(
            "subscription already exists",
            subscription_path=subscription_path,
            error_message=str(e),
            error_type=type(e).__name__,
        )

    subscriber_paths.add(subscription_path)
    return subscription_path


async def publish_notification[T](
    *,
    parent_id: UUID,
    event: str,
    data: T,
    trace_id: str,
) -> str:
    client = get_publisher_client()

    topic_path = client.topic_path(
        project=get_env("GCP_PROJECT_ID", fallback="grantflow"),
        topic=get_env("FRONTEND_NOTIFICATIONS_PUBSUB_TOPIC", fallback="frontend-notifications"),
    )

    logger.info(
        "Publishing notification",
        topic_path=topic_path,
        notification_event=event,
        parent_id=str(parent_id),
        trace_id=trace_id,
    )
    try:
        websocket_message = WebsocketMessage(
            event=event,
            data=data,
            parent_id=parent_id,
            type="data",
            trace_id=trace_id,
        )

        message_data = serialize(websocket_message)

        with create_pubsub_publish_span(topic_path, "WebsocketMessage") as span:
            span.set_attribute("event", event)
            span.set_attribute("parent_id", str(parent_id))
            if trace_id:
                span.set_attribute("trace_id", trace_id)

            attributes = {
                "parent_id": str(parent_id),
            }
            if trace_id:
                attributes["trace_id"] = trace_id
            attributes = inject_trace_context(attributes)

            future = client.publish(topic_path, message_data, **attributes)
            message_id = await run_sync(future.result)

            span.set_attribute("messaging.message.id", message_id)

        logger.info(
            "Published notification",
            message_id=message_id,
            trace_id=trace_id,
        )
        return str(message_id)
    except MessageTooLargeError as e:
        logger.error("Error publishing notification", error=str(e))
        raise BackendError("Error publishing notification", context={"error": str(e)}) from e


async def publish_email_notification(
    *,
    application_id: str | UUID,
    trace_id: str,
) -> str:
    client = get_publisher_client()

    message_data = b""
    topic_path = client.topic_path(
        project=get_env("GCP_PROJECT_ID", fallback="grantflow"),
        topic=get_env("EMAIL_NOTIFICATIONS_PUBSUB_TOPIC", fallback="email-notifications"),
    )

    try:
        with create_pubsub_publish_span(topic_path, "EmailNotificationRequest") as span:
            span.set_attribute("application_id", str(application_id))
            if trace_id:
                span.set_attribute("trace_id", trace_id)

            attributes = {"application_id": str(application_id)}
            if trace_id:
                attributes["trace_id"] = trace_id

            attributes = inject_trace_context(attributes)

            future = client.publish(topic_path, message_data, **attributes)
            message_id = await run_sync(future.result)

            span.set_attribute("messaging.message.id", message_id)

        logger.info(
            "Published email notification message",
            message_id=message_id,
            application_id=str(application_id),
            trace_id=trace_id,
        )
        return str(message_id)
    except MessageTooLargeError as e:
        logger.error("Error publishing email notification message", error=str(e))
        raise BackendError("Error publishing email notification message", context={"error": str(e)}) from e


async def publish_subscription_verification_email(
    *,
    email: str,
    subscription_id: str,
    verification_token: str,
    search_params: dict[str, Any] | None = None,
    frequency: str = "daily",
    trace_id: str,
) -> str:
    client = get_publisher_client()

    data = SubscriptionVerificationRequest(
        email=email,
        subscription_id=subscription_id,
        verification_token=verification_token,
        template_type="subscription_verification",
        trace_id=trace_id,
    )

    if search_params:
        data["search_params"] = search_params
    if frequency:
        data["frequency"] = frequency

    try:
        message_data = serialize(data)
        topic_path = client.topic_path(
            project=get_env("GCP_PROJECT_ID", fallback="grantflow"),
            topic=get_env("EMAIL_NOTIFICATIONS_PUBSUB_TOPIC", fallback="email-notifications"),
        )

        with create_pubsub_publish_span(topic_path, "SubscriptionVerificationRequest") as span:
            span.set_attribute("email", email)
            span.set_attribute("subscription_id", subscription_id)
            if trace_id:
                span.set_attribute("trace_id", trace_id)

            attributes = {"notification_type": "subscription_verification"}
            if trace_id:
                attributes["trace_id"] = trace_id
            attributes = inject_trace_context(attributes)

            future = client.publish(topic_path, message_data, **attributes)
            message_id = await run_sync(future.result)

            span.set_attribute("messaging.message.id", message_id)

        logger.info(
            "Published subscription verification email",
            message_id=message_id,
            email=email,
            subscription_id=subscription_id,
            trace_id=trace_id,
        )
        return str(message_id)
    except MessageTooLargeError as e:
        logger.error("Error publishing subscription verification email", error=str(e))
        raise BackendError(
            "Error publishing subscription verification email",
            context={"error": str(e)},
        ) from e


async def pull_notifications(
    *,
    parent_id: UUID,
) -> list[WebsocketMessage[dict[str, Any]]]:
    start_time = time.time()
    client = get_subscriber_client()

    subscription_path = await ensure_subscription_for_parent_id(parent_id)
    logger.info(
        "pulling notifications",
        subscription_path=subscription_path,
        parent_id=str(parent_id),
    )

    try:
        response = await run_sync(
            client.pull,
            request={
                "subscription": subscription_path,
                "max_messages": 100,
            },
            timeout=3.0,
        )
    except Exception as e:
        pull_duration = time.time() - start_time
        logger.warning(
            "Failed to pull messages from subscription",
            subscription_path=subscription_path,
            parent_id=str(parent_id),
            error=str(e),
            pull_duration_ms=round(pull_duration * 1000, 2),
        )
        raise

    ret: list[WebsocketMessage[dict[str, Any]]] = []
    ack_ids: list[str] = []

    logger.info(
        "received pubsub response",
        subscription_path=subscription_path,
        parent_id=str(parent_id),
        num_messages=len(response.received_messages),
    )

    for received_message in response.received_messages:
        try:
            message = deserialize(received_message.message.data, WebsocketMessage[dict[str, Any]])
            logger.debug(
                "received message from pubsub",
                message_id=received_message.message.message_id,
                publish_time=received_message.message.publish_time.isoformat()
                if received_message.message.publish_time
                else None,
                attributes=dict(received_message.message.attributes),
                parent_id=str(parent_id),
            )
            ret.append(message)
            ack_ids.append(received_message.ack_id)
        except (DeserializationError, ValueError, KeyError, TypeError) as e:
            logger.error(
                "Error processing notification",
                error=str(e),
                message_id=received_message.message.message_id,
                parent_id=str(parent_id),
                raw_data=received_message.message.data[:200] if received_message.message.data else None,
            )
            ack_ids.append(received_message.ack_id)

    if ack_ids:
        logger.info(
            "acking messages",
            num_acks=len(ack_ids),
            parent_id=str(parent_id),
            subscription_path=subscription_path,
        )
        try:
            await run_sync(
                client.acknowledge,
                request={
                    "subscription": subscription_path,
                    "ack_ids": ack_ids,
                },
            )
            pull_duration = time.time() - start_time
            logger.info(
                "Successfully acknowledged messages",
                num_acks=len(ack_ids),
                parent_id=str(parent_id),
                pull_duration_ms=round(pull_duration * 1000, 2),
            )
        except Exception as e:
            logger.error(
                "Failed to acknowledge messages",
                error=str(e),
                num_acks=len(ack_ids),
                parent_id=str(parent_id),
            )
            raise

    return ret
