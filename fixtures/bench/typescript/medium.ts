import ky, { type KyInstance } from "ky";
import { getEnv } from "@/utils/env";
import { log } from "@/utils/logger/client";
import { Ref } from "@/utils/state";
import { generateTraceId } from "@/utils/tracing";

const clientRef = new Ref<KyInstance>();

const ONE_MINUTE_IN_MS = 60 * 1000;

export function getClient(): KyInstance {
	const backendUrl = getEnv().NEXT_PUBLIC_BACKEND_API_BASE_URL;

	if (!clientRef.value) {
		log.info("Initializing API client", {
			backend_url: backendUrl,
			environment: process.env.NODE_ENV,
		});
	}

	clientRef.value ??= ky.create({
		hooks: {
			afterResponse: [
				async (request, _options, response, state) => {
					const clonedResponse = response.clone();
					let responseBody: unknown;

					try {
						const contentType = response.headers.get("content-type");
						if (contentType?.includes("application/json")) {
							responseBody = await clonedResponse.json();
						}
					} catch {}

					log.info(`API ${request.method} ${request.url} - ${response.status}`, {
						method: request.method,
						operation: request.headers.get("X-Operation"),
						response_body: responseBody,
						response_headers: Object.fromEntries(response.headers.entries()),
						retry_count: state.retryCount,
						status: response.status,
						trace_id: request.headers.get("X-Trace-ID"),
						url: request.url,
					});

					return response;
				},
			],
			beforeError: [
				async (error, state) => {
					let responseBody: unknown;
					let responseText: string | undefined;

					try {
						const clonedResponse = error.response.clone();
						const contentType = error.response.headers.get("content-type");

						if (contentType?.includes("application/json")) {
							responseBody = await clonedResponse.json();
						} else {
							responseText = await clonedResponse.text();
						}
					} catch {}

					log.error(`API ERROR ${error.request.method} ${error.request.url}`, error, {
						backend_url: backendUrl,
						error_message: error.message,
						method: error.request.method,
						operation: error.request.headers.get("X-Operation"),
						request_headers: Object.fromEntries(error.request.headers.entries()),
						response_body: responseBody,
						response_headers: Object.fromEntries(error.response.headers.entries()),
						response_text: responseText,
						retry_count: state.retryCount,
						status: error.response.status,
						status_text: error.response.statusText,
						trace_id: error.request.headers.get("X-Trace-ID"),
						url: error.request.url,
					});

					return error;
				},
			],
			beforeRequest: [
				(request, options, state) => {
					if (!request.headers.get("X-Trace-ID")) {
						const traceId = generateTraceId();
						request.headers.set("X-Trace-ID", traceId);
						log.info("Auto-generated trace_id for request", {
							method: request.method,
							trace_id: traceId,
							url: request.url,
						});
					}

					let requestBody: unknown;
					if (options.body) {
						try {
							requestBody = typeof options.body === "string" ? JSON.parse(options.body) : options.body;
						} catch {
							requestBody = options.body;
						}
					}

					log.info(`API REQUEST ${request.method} ${request.url}`, {
						base_url: backendUrl,
						full_url: new URL(request.url).href,
						method: request.method,
						operation: request.headers.get("X-Operation"),
						pathname: new URL(request.url).pathname,
						request_body: requestBody,
						request_headers: Object.fromEntries(request.headers.entries()),
						retry_count: state.retryCount,
						trace_id: request.headers.get("X-Trace-ID"),
						url: request.url,
					});
				},
			],
		},
		prefixUrl: backendUrl,
		timeout: ONE_MINUTE_IN_MS * 10,
	});

	return clientRef.value;
}
