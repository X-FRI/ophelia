module Parser = Parser
module Type = Type
module Eval = Eval
module Ast = Ast

let interop s =
    let ast = Parser.parse s in
        Type.check ast;
        Eval.eval_big ast
