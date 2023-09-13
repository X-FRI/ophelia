(* The MIT License (MIT)
 * 
 * Copyright (c) 2022 Muqiu Han
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *)

type expr =
  | Identifier of string
  | Literal of literal
  | Define of define
  | Prim of string * expr * expr
  | If of expr * expr * expr
  | Call of expr * expr
  | Type of type_expr

and literal =
  | Literal_bool of bool
  | Literal_int of int

and define =
  | Define_var of string * expr * expr
  | Define_fun of string * string * expr * expr * expr * expr

and type_expr =
  | Type_int
  | Type_bool
  | Type_fun of type_expr * type_expr

type value =
  | Value_int of int
  | Value_closure of string * string * expr * value Env.t

let type_of_id = function
    | Identifier "Int" -> Type Type_int
    | Identifier "Bool" -> Type Type_bool
    | Identifier t ->
        failwith (Format.sprintf "type_of_string: unknown type %s" t)
    | _ -> failwith "type_of_id: not a type expr"

module Infer = struct
  let rec infer e env =
      match e with
      | Identifier x -> Env.lookup env x
      | Literal l -> infer_literal l
      | Define def -> infer_define def env
      | Prim _ as prim -> infer_prim prim env
      | Call _ as call -> infer_call call env
      | _ -> failwith "Infer: fail"

  and infer_literal = function
      | Literal_int _ -> Type_int
      | Literal_bool _ -> Type_bool

  and infer_prim prim env =
      match prim with
      | Prim (op, e1, e2) -> (
          let t1 = infer e1 env in
          let t2 = infer e2 env in
              match (op, t1, t2) with
              | "*", Type_int, Type_int -> Type_int
              | "+", Type_int, Type_int -> Type_int
              | "-", Type_int, Type_int -> Type_int
              | "=", Type_int, Type_int -> Type_bool
              | "<", Type_int, Type_int -> Type_bool
              | "&", Type_bool, Type_bool -> Type_bool
              | _ -> failwith "unknown op, or type error")
      | _ -> failwith "prim"

  and infer_define def env =
      match def with
      | Define_var (x, erhs, body) ->
          let x_type = infer erhs env in
          let body_env = (x, x_type) :: env in
              infer body body_env
      | Define_fun (f, x, Type x_type, f_body, Type r_type, body) ->
          let f_type = Type_fun (x_type, r_type) in
          let f_body_env = (x, x_type) :: (f, f_type) :: env in
          let body_env = (f, f_type) :: env in
              if infer f_body f_body_env = r_type then
                infer body body_env
              else
                failwith (Format.sprintf "Define_fun: return type in %s" f)
      | _ -> failwith (Format.sprintf "Define_fun: type error")

  and infer_call call env =
      match call with
      | Call (Identifier f, args) -> begin
          match Env.lookup env f with
          | Type_fun (x_type, r_type) ->
              if infer args env = x_type then
                r_type
              else
                failwith "Call: wrong argument type"
          | _ -> failwith "Call: unknown function"
        end
      | _ -> failwith "call"
end
