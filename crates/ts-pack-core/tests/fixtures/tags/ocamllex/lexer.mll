{
  open Parser
  exception Eof
}

let digit = ['0'-'9']
let ident = ['a'-'z' '_'] ['a'-'z' '0'-'9' '_']*

rule token = parse
  | [' ' '\t' '\n'] { token lexbuf }
  | digit+ as n { INT (int_of_string n) }
  | ident as id { IDENT id }
  | '+' { PLUS }
  | "(*" { comment lexbuf; token lexbuf }
  | eof { raise Eof }

and comment = parse
  | "*)" { () }
  | _ { comment lexbuf }
