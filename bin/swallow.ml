open Swallow_lib

let _ =
    let repl () =
        print_endline "> ";
        let ast = Parser.parse (input_line stdin) in
            Eval.eval ast []
    in

    repl ()
