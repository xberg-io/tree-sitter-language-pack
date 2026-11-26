// ~keep Modified by tree-sitter-language-pack (see ATTRIBUTIONS.md): adds scan_spaced_operator.
#include "tree_sitter/alloc.h"
#include "tree_sitter/parser.h"
#include <stdio.h>
#include <wctype.h>

enum TokenType {
    TEXT_LITERAL_BEGIN,
    TEXT_LITERAL_CONTENT,
    TEXT_LITERAL_END,
    BLOCK_COMMENT_BEGIN,
    BLOCK_COMMENT_CONTENT,
    BLOCK_COMMENT_END,
    LINE_COMMENT_BEGIN,
    LINE_COMMENT_CONTENT,
    DOC_COMMENT_BEGIN,
    DOC_COMMENT_CONTENT,
    SPACED_GT,
    SPACED_LT,
    SPACED_SHR,
};

enum MotokoScannerMode {
    NONE = 1,
    PARSE_TEXT,
    PARSE_BLOCK_COMMENT,
    PARSE_LINE_COMMENT,
    PARSE_DOC_COMMENT
};

struct MotokoScanner {
    enum MotokoScannerMode mode;
    uint32_t block_comment_depth;
};

static const size_t MOSCSIZE = sizeof(struct MotokoScanner);

void *tree_sitter_motoko_external_scanner_create() {
    struct MotokoScanner *mosc = ts_malloc(MOSCSIZE);

    mosc->mode = NONE;
    mosc->block_comment_depth = 0;

    return mosc;
}
void tree_sitter_motoko_external_scanner_destroy(void *p) {
    ts_free(p);
}
void tree_sitter_motoko_external_scanner_reset(void *p) {
    struct MotokoScanner *mosc = p;

    mosc->mode = NONE;
    mosc->block_comment_depth = 0;
}
unsigned tree_sitter_motoko_external_scanner_serialize(void *p, char *buffer) {
    struct MotokoScanner *mosc = p;

    buffer[0] = mosc->mode;
    buffer[1] = mosc->block_comment_depth;

    return MOSCSIZE;
}
void tree_sitter_motoko_external_scanner_deserialize(void *p, const char *buffer, unsigned n) {
    if (n == MOSCSIZE) {
        struct MotokoScanner *mosc = p;

        mosc->mode = buffer[0];
        mosc->block_comment_depth = buffer[1];
    }
}

static void advance(TSLexer *lexer) {
    lexer->advance(lexer, false);
}

static bool is_num_char(int32_t c) {
    return c == '_' || iswdigit(c);
}

// ~keep Binary `>`, `<` and `>>` (upstream's " > ", " < " and " >>" tokens). Called after
// ~keep whitespace was skipped, so the token holds only the operator. `>`/`<` also need
// ~keep whitespace after them, and `>>` must not be the start of `>>=`.
static bool scan_spaced_operator(TSLexer *lexer, const bool *valid_symbols) {
    if (lexer->lookahead == '<' && valid_symbols[SPACED_LT]) {
        advance(lexer);
        if (!iswspace(lexer->lookahead)) {
            return false;
        }
        lexer->result_symbol = SPACED_LT;
        return true;
    }
    if (lexer->lookahead != '>' || !(valid_symbols[SPACED_GT] || valid_symbols[SPACED_SHR])) {
        return false;
    }
    advance(lexer);
    if (lexer->lookahead == '>') {
        if (!valid_symbols[SPACED_SHR]) {
            return false;
        }
        advance(lexer);
        if (lexer->lookahead == '=') {
            return false;
        }
        lexer->result_symbol = SPACED_SHR;
        return true;
    }
    if (!valid_symbols[SPACED_GT] || !iswspace(lexer->lookahead)) {
        return false;
    }
    lexer->result_symbol = SPACED_GT;
    return true;
}

static bool process_text(struct MotokoScanner *mosc, TSLexer *lexer, const bool *valid_symbols) {
    if (lexer->eof(lexer)) {
        return false;
    }
    // printf("hello text\n");

    if (valid_symbols[TEXT_LITERAL_END] && lexer->lookahead == '"') {
        //   printf("goodbye\n");
        lexer->result_symbol = TEXT_LITERAL_END;
        lexer->advance(lexer, false);
        mosc->mode = NONE;

        return true;
    } else if (valid_symbols[TEXT_LITERAL_CONTENT]) {
        //   printf("uhh parse something\n");
        if (lexer->lookahead == '\\') {
            return false;
        }
        lexer->result_symbol = TEXT_LITERAL_CONTENT;
        do {
            //     printf("> hello %d\n", lexer->lookahead);
            lexer->advance(lexer, false);
        } while (!lexer->eof(lexer) && lexer->lookahead != '"' && lexer->lookahead != '\\');

        return true;
    }

    return false;
}

static bool process_block_comment(struct MotokoScanner *mosc, TSLexer *lexer,
                                  const bool *valid_symbols) {
    if (lexer->eof(lexer)) {
        return false;
    }
    if (valid_symbols[BLOCK_COMMENT_END] && lexer->lookahead == '*' &&
        mosc->block_comment_depth == 1) {
        lexer->mark_end(lexer);
        lexer->advance(lexer, false);

        if (lexer->lookahead == '/') {
            lexer->result_symbol = BLOCK_COMMENT_END;
            lexer->advance(lexer, false);
            lexer->mark_end(lexer);
            mosc->mode = NONE;
            mosc->block_comment_depth = 0;

            return true;
        }
    }

    if (valid_symbols[BLOCK_COMMENT_CONTENT]) {
        lexer->result_symbol = BLOCK_COMMENT_CONTENT;
    get_content:
        do {
            lexer->advance(lexer, false);
            if (lexer->eof(lexer)) {
                goto outer;
            }
        } while (lexer->lookahead != '*' && lexer->lookahead != '/');
        if (lexer->lookahead == '*') {
            if (mosc->block_comment_depth == 1) {
                lexer->mark_end(lexer);
            }
            lexer->advance(lexer, false);

            if (lexer->eof(lexer)) {
                goto outer;
            } else if (lexer->lookahead == '/') {
                if (mosc->block_comment_depth > 1) {
                    mosc->block_comment_depth--;
                } else {
                    goto outer_no_mark;
                }
            }
        } else /* if (lexer->lookahead == '/') */ {
            lexer->advance(lexer, false);

            if (lexer->eof(lexer)) {
                goto outer;
            } else if (lexer->lookahead == '*') {
                mosc->block_comment_depth++;
            }
        }
        goto get_content;
    outer:
        lexer->mark_end(lexer);
    outer_no_mark:

        return true;
    }

    return false;
}

#define LINELIKE_COMMENT_PROCESSOR(PRENAME, PRENAME_CAP)                                           \
    static bool process_##PRENAME##_comment(struct MotokoScanner *mosc, TSLexer *lexer,            \
                                            const bool *valid_symbols) {                           \
        if (lexer->eof(lexer)) {                                                                   \
            return false;                                                                          \
        }                                                                                          \
                                                                                                   \
        if (valid_symbols[PRENAME_CAP##_COMMENT_CONTENT]) {                                        \
            lexer->result_symbol = PRENAME_CAP##_COMMENT_CONTENT;                                  \
            do {                                                                                   \
                lexer->advance(lexer, false);                                                      \
            } while (!lexer->eof(lexer) && lexer->lookahead != '\n');                              \
                                                                                                   \
            mosc->mode = NONE;                                                                     \
                                                                                                   \
            return true;                                                                           \
        }                                                                                          \
                                                                                                   \
        return false;                                                                              \
    }

LINELIKE_COMMENT_PROCESSOR(line, LINE)
LINELIKE_COMMENT_PROCESSOR(doc, DOC)

#define SIMPLE_SCANNER_MODE_CASE(MODE, MODE_TARGET)                                                \
    case MODE: {                                                                                   \
        if (process_##MODE_TARGET(mosc, lexer, valid_symbols)) {                                   \
            return true;                                                                           \
        }                                                                                          \
        break;                                                                                     \
    }

bool tree_sitter_motoko_external_scanner_scan(void *payload, TSLexer *lexer,
                                              const bool *valid_symbols) {
    struct MotokoScanner *mosc = payload;

    switch (mosc->mode) {
        SIMPLE_SCANNER_MODE_CASE(PARSE_TEXT, text)
        SIMPLE_SCANNER_MODE_CASE(PARSE_BLOCK_COMMENT, block_comment)
        SIMPLE_SCANNER_MODE_CASE(PARSE_LINE_COMMENT, line_comment)
        SIMPLE_SCANNER_MODE_CASE(PARSE_DOC_COMMENT, doc_comment)
    default: {
        bool skipped_space = false;
        while (!lexer->eof(lexer) && iswspace(lexer->lookahead)) {
            lexer->advance(lexer, true);
            skipped_space = true;
        }

        if (skipped_space && scan_spaced_operator(lexer, valid_symbols)) {
            return true;
        }

        if (valid_symbols[TEXT_LITERAL_BEGIN] && lexer->lookahead == '"') {
            lexer->advance(lexer, false);
            lexer->result_symbol = TEXT_LITERAL_BEGIN;
            mosc->mode = PARSE_TEXT;
            return true;
        }

        const bool is_line_comment_valid =
            valid_symbols[LINE_COMMENT_BEGIN] || valid_symbols[DOC_COMMENT_BEGIN];
        const bool is_any_comment_valid =
            valid_symbols[BLOCK_COMMENT_BEGIN] || is_line_comment_valid;

        if (is_any_comment_valid && lexer->lookahead == '/') {
            lexer->mark_end(lexer);
            lexer->advance(lexer, false);
            if (valid_symbols[BLOCK_COMMENT_BEGIN] && lexer->lookahead == '*') {
                lexer->result_symbol = BLOCK_COMMENT_BEGIN;
                lexer->advance(lexer, false);
                lexer->mark_end(lexer);
                mosc->mode = PARSE_BLOCK_COMMENT;
                mosc->block_comment_depth++;

                return true;
            } else if (is_line_comment_valid && lexer->lookahead == '/') {
                lexer->advance(lexer, false);
                if (valid_symbols[DOC_COMMENT_BEGIN] && lexer->lookahead == '/') {
                    lexer->result_symbol = DOC_COMMENT_BEGIN;
                    lexer->advance(lexer, false);
                    lexer->mark_end(lexer);
                    mosc->mode = PARSE_DOC_COMMENT;

                    return true;
                } else if (valid_symbols[LINE_COMMENT_BEGIN]) {
                    lexer->result_symbol = LINE_COMMENT_BEGIN;
                    lexer->mark_end(lexer);
                    mosc->mode = PARSE_LINE_COMMENT;

                    return true;
                }
            }
        }
    }
    }

    return false;
}
