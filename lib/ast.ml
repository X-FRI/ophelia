type binary_operator =
  | Add
  | Mult
  | Leq

type expr =
  | Var of string
  | Int of int
  | Bool of bool
  | Binop of binary_operator * expr * expr
  | Let of string * expr * expr
  | If of expr * expr * expr
