type bop =
  | Add
  | Mult
  | Leq
[@@deriving show]

type expr =
  | Var of string
  | Int of int
  | Bool of bool
  | Binop of bop * expr * expr
  | Let of string * expr * expr
  | If of expr * expr * expr
[@@deriving show]
