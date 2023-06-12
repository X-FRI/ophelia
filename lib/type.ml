open Ast

type typ =
  | TInt
  | TBool

and t = typ

(** [typeof ctx e] is the type of [e] in context [ctx]. 
    Raises: [Failure] if [e] is not well typed in [ctx]. *)
let rec typeof ctx = function
    | Int _ -> TInt
    | Bool _ -> TBool
    | Var x -> Context.lookup ctx x
    | Let (x, e1, e2) -> typeof_let ctx x e1 e2
    | Binop (bop, e1, e2) -> typeof_bop ctx bop e1 e2
    | If (e1, e2, e3) -> typeof_if ctx e1 e2 e3

(** Helper function for [typeof]. *)
and typeof_let ctx x e1 e2 =
    let t1 = typeof ctx e1 in
    let ctx' = Context.extend ctx x t1 in
        typeof ctx' e2

(** Helper function for [typeof]. *)
and typeof_bop ctx bop e1 e2 =
    let t1, t2 = (typeof ctx e1, typeof ctx e2) in
        match (bop, t1, t2) with
        | Add, TInt, TInt | Mult, TInt, TInt -> TInt
        | Leq, TInt, TInt -> TBool
        | _ -> failwith Error.bop_err

(** Helper function for [typeof]. *)
and typeof_if ctx e1 e2 e3 =
    if typeof ctx e1 = TBool then begin
      let t2 = typeof ctx e2 in
          if t2 = typeof ctx e3 then
            t2
          else
            failwith Error.if_branch_err
    end else
      failwith Error.if_guard_err

(** [check e] checks whether [e] is well typed in
      the empty context. Raises: [Failure] if not. *)
let check e = ignore (typeof Context.empty e)
