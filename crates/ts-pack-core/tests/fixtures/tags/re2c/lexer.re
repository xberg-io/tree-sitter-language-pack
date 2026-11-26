#include <stdio.h>

/*!re2c
    re2c:define:YYCTYPE = char;
    re2c:yyfill:enable = 0;

    digit = [0-9];
    number = digit+;
*/

/*!rules:re2c:tokens
    ident = [a-zA-Z_][a-zA-Z_0-9]*;

    number { return 1; }
    ident  { return 2; }
    *      { return 0; }
*/

static int lex(const char *YYCURSOR)
{
    const char *YYMARKER;
    /*!use:re2c:tokens */
}

static int lex_local(const char *YYCURSOR)
{
    /*!local:re2c
        space = [ \t]+;
        space { return 3; }
        *     { return 0; }
    */
}

int main(void)
{
    printf("%d\n", lex("42"));
    return 0;
}
