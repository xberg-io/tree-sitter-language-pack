#include <stdio.h>
#include <stdlib.h>

#ifndef SCANNER_SOURCE
#define SCANNER_SOURCE "../../grammars/reason/src/scanner.c"
#endif
#include SCANNER_SOURCE

typedef struct {
  TSLexer lexer;
  const char *input;
  unsigned position;
  unsigned skipped;
  unsigned marked;
  unsigned steps;
} TestLexer;

static void test_advance(TSLexer *lexer, bool skip) {
  TestLexer *test = (TestLexer *)lexer;
  if (++test->steps > 4096)
    abort();
  if (test->input[test->position] != '\0') {
    ++test->position;
    if (skip)
      ++test->skipped;
  }
  lexer->lookahead = (unsigned char)test->input[test->position];
}

static void test_mark_end(TSLexer *lexer) {
  TestLexer *test = (TestLexer *)lexer;
  test->marked = test->position;
}

static bool test_eof(const TSLexer *lexer) {
  const TestLexer *test = (const TestLexer *)lexer;
  return test->input[test->position] == '\0';
}

int main(int argc, char **argv) {
  if (argc != 6)
    return EXIT_FAILURE;
  TestLexer test = {0};
  test.input = argv[1];
  test.lexer.lookahead = (unsigned char)test.input[0];
  test.lexer.result_symbol = 99;
  test.lexer.advance = test_advance;
  test.lexer.mark_end = test_mark_end;
  test.lexer.eof = test_eof;
  unsigned mask = (unsigned)strtoul(argv[2], NULL, 10);
  unsigned flags = (unsigned)strtoul(argv[3], NULL, 10);
  unsigned repeats = (unsigned)strtoul(argv[5], NULL, 10);
  bool valid_symbols[8] = {false};
  for (unsigned i = 0; i < 8; ++i)
    valid_symbols[i] = (mask & (1u << i)) != 0;
  ScannerState *state = tree_sitter_reason_external_scanner_create();
  state->parens_nesting = (int)strtol(argv[4], NULL, 10);
  state->in_quotes = (flags & 1) != 0;
  state->in_backticks = (flags & 2) != 0;
  state->eof_reported = (flags & 4) != 0;
  for (unsigned i = 0; i < repeats; ++i) {
    bool accepted = tree_sitter_reason_external_scanner_scan(state, &test.lexer,
                                                             valid_symbols);
    printf("%d %u %u %u %u %d %d %d %d\n", accepted, test.position,
           test.skipped, test.marked, test.lexer.result_symbol,
           state->parens_nesting, state->in_quotes, state->in_backticks,
           state->eof_reported);
  }
  tree_sitter_reason_external_scanner_destroy(state);
  return EXIT_SUCCESS;
}
