type expr =
  | CstI of int
  | CstB of bool
  | Var of string
  | Let of string * expr * expr
  | Prim of string * expr * expr
  | If of expr * expr * expr
  | Fun of string * string * expr * expr
  | Call of expr * expr

type value =
  | Int of int
  | Closure of string * string * expr * value Env.t
