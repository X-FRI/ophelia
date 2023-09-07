module Parser = Parser
module Eval = Eval
module Ast = Ast
module Error = Error

let interop s =
    let ast = Parser.parse s in
        Eval.eval ast
