from __future__ import annotations

import math
from functools import lru_cache
from typing import Any, Final

from anthropic import AsyncAnthropic
from google import genai
from google.oauth2.service_account import Credentials
from packages.shared_utils.src.env import get_env
from packages.shared_utils.src.exceptions import BackendError
from packages.shared_utils.src.ref import Ref
from packages.shared_utils.src.serialization import deserialize

EVALUATION_MODEL: Final[str] = get_env("EVALUATION_MODEL", fallback="gemini-flash-latest")
GENERATION_MODEL: Final[str] = get_env("GENERATION_MODEL", fallback="gemini-flash-latest")
ANTHROPIC_SONNET_MODEL: Final[str] = get_env("ANTHROPIC_SONNET_MODEL", fallback="claude-sonnet-4-20250514")
REASONING_MODEL: Final[str] = get_env("REASONING_MODEL", fallback="gemini-flash-latest")

GEMINI_FLASH_MODEL: Final[str] = "gemini-flash-latest"

init_ref = Ref[bool]()
anthropic_client = Ref[AsyncAnthropic]()
google_client = Ref[genai.Client | None]()


def get_vertex_credentials() -> Credentials:
    credentials = deserialize(get_env("LLM_SERVICE_ACCOUNT_CREDENTIALS"), dict[str, Any])

    scopes = ["https://www.googleapis.com/auth/cloud-platform"]
    return Credentials.from_service_account_info(credentials, scopes=scopes)  # type: ignore[no-any-return, no-untyped-call]


def init_llm_connection() -> None:
    if not init_ref.value:
        google_client.value = genai.Client(
            api_key=get_env("GOOGLE_AI_API_KEY"),
        )
        init_ref.value = True


def get_google_ai_client() -> genai.Client:
    if not google_client.value:
        init_llm_connection()
    return google_client.value  # type: ignore[return-value]


def get_anthropic_client() -> AsyncAnthropic:
    if not anthropic_client.value:
        anthropic_client.value = AsyncAnthropic(
            api_key=get_env("ANTHROPIC_API_KEY"),
        )
    return anthropic_client.value


async def count_tokens(text: str, model: str = ANTHROPIC_SONNET_MODEL) -> int:
    if not text:
        return 0

    if ANTHROPIC_SONNET_MODEL in model:
        return estimate_token_count(text)

    try:
        client = get_google_ai_client()
        response = await client._aio.models.count_tokens(  # noqa: SLF001
            model=model,
            contents=text,
        )
        return int(response.total_tokens or 0)
    except (ValueError, KeyError, AttributeError):
        return estimate_token_count(text)


CHARS_PER_TOKEN: Final[float] = 4.0


@lru_cache(maxsize=1000)
def estimate_token_count(text: str) -> int:
    try:
        from packages.shared_utils.src.nlp import get_word_count

        if not text:
            return 0

        if len(text) < 100:
            return math.ceil(len(text) / CHARS_PER_TOKEN)

        word_count = get_word_count(text)
        char_count = len(text)

        char_tokens = char_count / CHARS_PER_TOKEN
        word_tokens = word_count * 1.3

        return math.ceil((char_tokens + word_tokens) / 2)

    except ImportError as e:
        raise BackendError("The NLP extra must be installed to use this function.") from e
