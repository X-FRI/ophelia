open Simpl

let _ =
    let rec repl () =
        print_endline "> ";
        let ast = Parser.parse (input_line stdin) in
            Type.check ast;
            Eval.eval_big ast |> Ast.show_expr |> print_endline |> repl
    in

    repl ()
