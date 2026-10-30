#include <stdio.h>
#include <stdlib.h>

#include "../../grammars/wolfram/src/scanner.c"

typedef struct {
  TSLexer lexer;
  const char *input;
  unsigned position;
  unsigned skipped;
} TestLexer;

static void test_advance(TSLexer *lexer, bool skip) {
  TestLexer *test = (TestLexer *)lexer;
  if (test->input[test->position] != '\0') {
    test->position++;
    if (skip)
      test->skipped++;
  }
  lexer->lookahead = (unsigned char)test->input[test->position];
}

int main(int argc, char **argv) {
  if (argc != 2)
    return EXIT_FAILURE;

  TestLexer test = {0};
  test.input = argv[1];
  test.lexer.lookahead = (unsigned char)test.input[0];
  test.lexer.result_symbol = 99;
  test.lexer.advance = test_advance;
  const bool valid_symbols[] = {true};
  bool accepted = tree_sitter_wolfram_external_scanner_scan(NULL, &test.lexer,
                                                            valid_symbols);
  printf("%d %u %u %u\n", accepted, test.position, test.skipped,
         test.lexer.result_symbol);
  return EXIT_SUCCESS;
}
