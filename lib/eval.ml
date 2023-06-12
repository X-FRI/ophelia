open Ast

(** [eval_small e] is the [e -->* v] relation.  That is,
    keep applying [step] until a value is produced.  *)
let rec eval_small (e : expr) : expr =
    if is_value e then
      e
    else
      e |> step |> eval_small

(** [interp_small s] interprets [s] by parsing, type-checking,
        and evaluating it with the small-step model. *)
let interp_small (s : string) : expr =
    let e = Parser.parse s in
        Type.check e;
        eval_small e

(** [eval_big e] is the [e ==> v] relation. *)
let rec eval_big (e : expr) : expr =
    match e with
    | Int _ | Bool _ -> e
    | Var _ -> failwith Error.unbound_var_err
    | Binop (bop, e1, e2) -> eval_bop bop e1 e2
    | Let (x, e1, e2) -> subst e2 (eval_big e1) x |> eval_big
    | If (e1, e2, e3) -> eval_if e1 e2 e3

(** [eval_bop bop e1 e2] is the [e] such that [e1 bop e2 ==> e]. *)
and eval_bop bop e1 e2 =
    match (bop, eval_big e1, eval_big e2) with
    | Add, Int a, Int b -> Int (a + b)
    | Mult, Int a, Int b -> Int (a * b)
    | Leq, Int a, Int b -> Bool (a <= b)
    | _ -> failwith Error.bop_err

(** [eval_if e1 e2 e3] is the [e] such that [if e1 then e2 else e3 ==> e]. *)
and eval_if e1 e2 e3 =
    match eval_big e1 with
    | Bool true -> eval_big e2
    | Bool false -> eval_big e3
    | _ -> failwith Error.if_guard_err
