module Parser = Parser
module Type = Type
module Eval = Eval

let interop s =
    let ast = Parser.parse s in
        Type.check ast;
        Eval.eval_big ast
