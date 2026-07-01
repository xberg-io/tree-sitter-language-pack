/*
 * Smoke test for tree-sitter-language-pack FFI.
 * Verifies that the installed shared library and headers work correctly.
 */
#include <ts_pack.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    /* Smoke test 1: verify ts_pack_available_languages is callable and returns a symbol. */
    const char *languages = ts_pack_available_languages();

    if (languages == NULL) {
        fprintf(stderr, "FAIL: ts_pack_available_languages returned NULL\n");
        return 1;
    }

    /* Smoke test 2: verify the result contains expected language names (e.g., "python"). */
    if (strstr(languages, "python") == NULL) {
        fprintf(stderr, "FAIL: expected 'python' in languages list, got: %s\n", languages);
        return 1;
    }

    printf("PASS: ts_pack_available_languages returned: %.*s (truncated)\n", 50, languages);
    return 0;
}
