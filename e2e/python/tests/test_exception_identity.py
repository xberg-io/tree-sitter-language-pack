"""Test GitHub issue #147: Python exception class identity and catching.

This test verifies that exception classes raised by the native module are the same
objects as those exported through the public Python API, ensuring that
`except DownloadError:` works correctly when catching exceptions from the native code.
"""

import pytest

from tree_sitter_language_pack import (
    CacheLockError,
    ChecksumMismatchError,
    ConfigError,
    DownloadError,
    DynamicLoadError,
    Error,
    InvalidRangeError,
    LanguageNotFoundError,
    LockPoisonedError,
    NullLanguagePointerError,
    ParseFailedError,
    ParserSetupError,
    QueryError,
)
from tree_sitter_language_pack._native import (
    CacheLockError as NCacheLockError,
    ChecksumMismatchError as NChecksumMismatchError,
    ConfigError as NConfigError,
    DownloadError as NDownloadError,
    DynamicLoadError as NDynamicLoadError,
    Error as NError,
    InvalidRangeError as NInvalidRangeError,
    LanguageNotFoundError as NLanguageNotFoundError,
    LockPoisonedError as NLockPoisonedError,
    NullLanguagePointerError as NNullLanguagePointerError,
    ParseFailedError as NParseFailedError,
    ParserSetupError as NParserSetupError,
    QueryError as NQueryError,
)


class TestExceptionIdentity:
    """Verify exception class objects are the same across import paths (issue #147)."""

    @pytest.mark.parametrize(
        "public_exc,native_exc,exc_name",
        [
            (Error, NError, "Error"),
            (LanguageNotFoundError, NLanguageNotFoundError, "LanguageNotFoundError"),
            (DynamicLoadError, NDynamicLoadError, "DynamicLoadError"),
            (NullLanguagePointerError, NNullLanguagePointerError, "NullLanguagePointerError"),
            (ParserSetupError, NParserSetupError, "ParserSetupError"),
            (LockPoisonedError, NLockPoisonedError, "LockPoisonedError"),
            (ConfigError, NConfigError, "ConfigError"),
            (ParseFailedError, NParseFailedError, "ParseFailedError"),
            (QueryError, NQueryError, "QueryError"),
            (InvalidRangeError, NInvalidRangeError, "InvalidRangeError"),
            (DownloadError, NDownloadError, "DownloadError"),
            (ChecksumMismatchError, NChecksumMismatchError, "ChecksumMismatchError"),
            (CacheLockError, NCacheLockError, "CacheLockError"),
        ],
    )
    def test_exception_class_identity(self, public_exc, native_exc, exc_name):
        """Test that exception class from public API is the same object as from _native."""
        assert (
            public_exc is native_exc
        ), f"{exc_name}: public import should be the same object as _native import"

    def test_download_error_catching(self):
        """Test that DownloadError can be caught using public API import (issue #147)."""
        import tree_sitter_language_pack

        # Attempting to get a non-existent language should raise DownloadError
        # (when using TSLP_LANGUAGES env var limiting available languages)
        with pytest.raises(DownloadError):
            tree_sitter_language_pack.get_language("nonexistent_language_xyz")

    def test_error_hierarchy_catching(self):
        """Test that catching base Error catches all variants."""
        import tree_sitter_language_pack

        # All specific errors are subclasses of Error, so catching Error should work
        with pytest.raises(Error):
            tree_sitter_language_pack.get_language("nonexistent_language_xyz")
