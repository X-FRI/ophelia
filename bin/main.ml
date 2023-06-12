open Sml

let _ =
    let ast = Parser.parse (input_line stdin) in
        Type.check ast;
        Eval.eval_big ast
