{
    open Swallow_parser
}

let white = [' ' '\t']+
let digit = ['0'-'9']
let int = '-'? digit+
let letter = ['a'-'z' 'A'-'Z']
let id = letter+

rule read =
  parse
  | white { read lexbuf }
  | "true" { TRUE }
  | "false" { FALSE }
  | "->" { RIGHT_ALLOW }
  | "{" { LEFT_CURLY }
  | "}" { RIGHT_CURLY }
  | "*" { MUL }
  | "+" { PLUS }
  | "/" { DIV }
  | "-" { SUB }
  | "(" { LPAREN }
  | ")" { RPAREN }
  | "let" { LET }
  | "fun" { FUN }
  | "=" { EQUALS }
  | "in" { IN }
  | "if" { IF }
  | "then" { THEN }
  | "else" { ELSE }
  | id { ID (Lexing.lexeme lexbuf) }
  | int { INT (int_of_string (Lexing.lexeme lexbuf)) }
  | eof { EOF }