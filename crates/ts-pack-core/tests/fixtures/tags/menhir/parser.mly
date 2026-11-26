%{
  open Ast
%}

%token <int> INT
%token PLUS TIMES LPAREN RPAREN EOF

%left PLUS
%left TIMES

%start <Ast.expr> main

%%

main:
  | e = expr EOF { e }

expr:
  | i = INT { Int i }
  | e1 = expr PLUS e2 = expr { Add (e1, e2) }
  | e1 = expr TIMES e2 = expr { Mul (e1, e2) }
  | LPAREN e = expr RPAREN { e }

let atom :=
  | i = INT; { Int i }
