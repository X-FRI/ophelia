module Parser = Parser
module Eval = Eval
module Ast = Ast
module Error = Error

let interop s =
    Parser.parse s
