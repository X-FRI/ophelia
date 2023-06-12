(** [parse s] parses [s] into an AST. *)
let parse (s : string) : Ast.expr =
    let lexbuf = Lexing.from_string s in
    let ast = Sml_parser.prog Sml_lexer.read lexbuf in
        ast
