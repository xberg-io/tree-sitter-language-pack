#include <string.h>
#include <tree_sitter/parser.h>
#include <wctype.h>

enum TokenType {
  NEWLINE,
  COMMENT,
  NEWLINE_AND_COMMENT,
  QUOTE,
  BACKTICK,
  TEMPLATE_CHARS,
  L_PAREN,
  R_PAREN,
};

typedef struct ScannerState {
  int parens_nesting;
  bool in_quotes;
  bool in_backticks;
  bool eof_reported;
} ScannerState;

void *tree_sitter_reason_external_scanner_create() {
  void *state = malloc(sizeof(ScannerState));
  memset(state, 0, sizeof(ScannerState));
  return state;
}

void tree_sitter_reason_external_scanner_destroy(void *state) { free(state); }

void tree_sitter_reason_external_scanner_reset(void *state) {
  memset(state, 0, sizeof(ScannerState));
}

unsigned tree_sitter_reason_external_scanner_serialize(void *state,
                                                       char *buffer) {
  memcpy(buffer, state, sizeof(ScannerState));
  return sizeof(ScannerState);
}

void tree_sitter_reason_external_scanner_deserialize(void *state,
                                                     const char *buffer,
                                                     unsigned n_bytes) {
  memcpy(state, buffer, n_bytes);
}

static void advance(TSLexer *lexer) { lexer->advance(lexer, false); }

static void scan_whitespace(TSLexer *lexer, bool skip) {
  while (iswspace(lexer->lookahead) && !lexer->eof(lexer)) {
    lexer->advance(lexer, skip);
  }
}

static void scan_multiline_comment(TSLexer *lexer) {
  int level = 1;
  advance(lexer);
  while (level > 0 && !lexer->eof(lexer)) {
    switch (lexer->lookahead) {
    case '/':
      advance(lexer);
      if (lexer->lookahead == '*')
        ++level;
      else
        continue;

      break;

    case '*':
      advance(lexer);
      if (lexer->lookahead == '/')
        --level;
      else
        continue;

      break;
    }

    advance(lexer);
  }
}

static bool scan_comment(TSLexer *lexer) {
  if (lexer->lookahead != '/')
    return false;

  advance(lexer);
  switch (lexer->lookahead) {
  case '/':
    // Single-line comment
    do {
      advance(lexer);
    } while (lexer->lookahead != '\n');
    return true;

  case '*':
    // Multi-line comment
    scan_multiline_comment(lexer);
    return true;

  default:
    // Division, etc
    return false;
  }
}

static bool scan_whitespace_and_comments(TSLexer *lexer) {
  bool has_comments = false;
  while (!lexer->eof(lexer)) {
    // Once a comment is found, the subsequent whitespace should not be marked
    // as skipped to keep the correct range of the comment node if it will be
    // marked so.
    bool skip_whitespace = !has_comments;
    scan_whitespace(lexer, skip_whitespace);
    if (scan_comment(lexer)) {
      has_comments = true;
    } else {
      break;
    }
  }

  return has_comments;
}

static bool is_identifier_start(char c) {
  return c == '_' || (c >= 'a' && c <= 'z');
}

static bool scan_template_chars(ScannerState *state, TSLexer *lexer) {
  lexer->result_symbol = TEMPLATE_CHARS;
  for (bool has_content = false;; has_content = true) {
    lexer->mark_end(lexer);
    switch (lexer->lookahead) {
    case '`':
      state->in_backticks = false;
      return has_content;
    case '\0':
      return false;
    case '$':
      advance(lexer);
      if (lexer->lookahead == '{' || is_identifier_start(lexer->lookahead))
        return has_content;
      break;
    case '\\':
      return has_content;
    default:
      advance(lexer);
    }
  }
}

static bool newline_continues_statement(TSLexer *lexer) {
  if (lexer->lookahead == '-') {
    advance(lexer);
    return lexer->lookahead == '>';
  }
  // ~keep Variant cases, ternaries and block closings continue the current
  // statement.
  if (lexer->lookahead == '|' || lexer->lookahead == '?' ||
      lexer->lookahead == ':' || lexer->lookahead == '}')
    return true;
  if (lexer->lookahead != 'a')
    return false;
  advance(lexer);
  if (lexer->lookahead != 'n')
    return false;
  advance(lexer);
  // ~keep `and` continues a recursive definition; leave its final character
  // unconsumed.
  return lexer->lookahead == 'd';
}

// ~keep EOF supplies one final statement-delimiting newline. Physical newlines
// can instead be formatting inside a statement. NEWLINE_AND_COMMENT preserves
// both statement-boundary semantics and comment highlighting when they
// coincide.
static bool scan_newline(ScannerState *state, TSLexer *lexer,
                         const bool *valid_symbols) {
  if (lexer->eof(lexer) && !state->eof_reported) {
    lexer->result_symbol = NEWLINE;
    state->eof_reported = true;
    return true;
  }
  if (lexer->lookahead != '\n')
    return false;
  lexer->result_symbol = NEWLINE;
  lexer->advance(lexer, true);
  lexer->mark_end(lexer);
  bool has_comment = scan_whitespace_and_comments(lexer);
  if (has_comment && valid_symbols[NEWLINE_AND_COMMENT]) {
    lexer->result_symbol = NEWLINE_AND_COMMENT;
    lexer->mark_end(lexer);
  }
  if (!newline_continues_statement(lexer))
    return true;
  if (has_comment && valid_symbols[COMMENT]) {
    lexer->result_symbol = COMMENT;
    return true;
  }
  return false;
}

static bool scan_delimiter(ScannerState *state, TSLexer *lexer,
                           const bool *valid_symbols) {
  if (valid_symbols[QUOTE] && lexer->lookahead == '"') {
    state->in_quotes = !state->in_quotes;
    lexer->result_symbol = QUOTE;
  } else if (valid_symbols[BACKTICK] && lexer->lookahead == '`') {
    state->in_backticks = !state->in_backticks;
    lexer->result_symbol = BACKTICK;
  } else if (valid_symbols[L_PAREN] && lexer->lookahead == '(') {
    ++state->parens_nesting;
    lexer->result_symbol = L_PAREN;
  } else if (valid_symbols[R_PAREN] && lexer->lookahead == ')') {
    --state->parens_nesting;
    lexer->result_symbol = R_PAREN;
  } else {
    return false;
  }
  lexer->advance(lexer, false);
  lexer->mark_end(lexer);
  return true;
}

bool tree_sitter_reason_external_scanner_scan(void *payload, TSLexer *lexer,
                                              const bool *valid_symbols) {
  ScannerState *state = (ScannerState *)payload;
  const bool in_string = state->in_quotes || state->in_backticks;
  if (valid_symbols[TEMPLATE_CHARS])
    return scan_template_chars(state, lexer);
  if (valid_symbols[NEWLINE] && scan_newline(state, lexer, valid_symbols))
    return true;
  if (!in_string)
    scan_whitespace(lexer, true);
  if (valid_symbols[COMMENT] && lexer->lookahead == '/' && !in_string) {
    lexer->result_symbol = COMMENT;
    if (scan_comment(lexer)) {
      lexer->mark_end(lexer);
      return true;
    }
    return false;
  }
  if (scan_delimiter(state, lexer, valid_symbols))
    return true;
  lexer->advance(lexer, iswspace(lexer->lookahead));
  return false;
}

// vim:sw=2
