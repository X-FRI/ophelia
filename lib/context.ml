(** A [Context] is a mapping from variable names to
    types, aka a symbol table, aka a typing environment. *)

(** [empty] is the empty context. *)
let empty = []

(** [lookup ctx x] gets the binding of [x] in [ctx]. 
      Raises: [Failure unbound_var_err] if [x] is
      not bound in [ctx]. *)
let lookup ctx x =
    try List.assoc x ctx with Not_found -> failwith Error.unbound_var_err

(** [extend ctx x ty] is [ctx] extended with a binding
      of [x] to [ty]. *)
let extend ctx x t = (x, t) :: ctx
