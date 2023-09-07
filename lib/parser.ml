let parse (s : string) : Ast.expr =
    let lexbuf = Lexing.from_string s in
    let ast = Swallow_parser.prog Swallow_lexer.read lexbuf in
        ast
