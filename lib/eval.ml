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

open Ast

let rec eval (e : Ast.expr) (env : Ast.value Env.t) : int =
    match e with
    | Identifier id -> eval_value id env
    | Literal literal -> eval_literal literal
    | Define def -> eval_def def env
    | Prim _ as prim -> eval_prim prim env
    | If (e1, e2, e3) ->
        let b = eval e1 env in
            if b <> 0 then
              eval e2 env
            else
              eval e3 env
    | Call (Identifier f, args) ->
        let closure = Env.lookup env f in
            begin
              match closure with
              | Value_closure (f, x, body, decl_env) ->
                  let x_val = Value_int (eval args env) in
                  let body_env = (x, x_val) :: (f, closure) :: decl_env in
                      eval body body_env
              | _ -> failwith "eval Call: not a function"
            end
    | Call _ -> failwith "eval Call: not first-order function"
    | Type _ -> failwith "eval Type: WTF??"

and eval_literal = function
    | Literal_int i -> i
    | Literal_bool b ->
        if b then
          1
        else
          0

and eval_value value env =
    match Env.lookup env value with
    | Value_int i -> i
    | _ -> failwith "eval Var"

and eval_def def env =
    match def with
    | Define_var (x, erhs, body) ->
        let xVal = Value_int (eval erhs env) in
        let bodyEnv = (x, xVal) :: env in
            eval body bodyEnv
    | Define_fun (f, x, _, f_body, _, body) as def ->
        Infer.infer_define def [] |> ignore;
        let body_env = (f, Value_closure (f, x, f_body, env)) :: env in
            eval body body_env

and eval_prim prim env =
    match prim with
    | Prim (op, e1, e2) ->
        let i1 = eval e1 env in
        let i2 = eval e2 env in
            begin
              match op with
              | "*" -> i1 * i2
              | "+" -> i1 + i2
              | "-" -> i1 - i2
              | "=" ->
                  if i1 = i2 then
                    1
                  else
                    0
              | "<" ->
                  if i1 < i2 then
                    1
                  else
                    0
              | _ -> failwith (Format.sprintf "unknown primitive %s" op)
            end
    | _ -> failwith "prim"
