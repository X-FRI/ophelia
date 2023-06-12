(** [parse s] parses [s] into an AST. *)
let parse (s : string) : Ast.expr =
    let lexbuf = Lexing.from_string s in
    let ast = Simpl_parser.prog Simpl_lexer.read lexbuf in
        ast
