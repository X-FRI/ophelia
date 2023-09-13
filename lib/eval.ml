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
    | CstI i -> i
    | CstB b ->
        if b then
          1
        else
          0
    | Var x -> begin
        match Env.lookup env x with
        | Int i -> i
        | _ -> failwith "eval Var"
      end
    | Prim (ope, e1, e2) ->
        let i1 = eval e1 env in
        let i2 = eval e2 env in
            begin
              match ope with
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
              | _ -> failwith (Format.sprintf "unknown primitive %s" ope)
            end
    | Let (x, eRhs, letBody) ->
        let xVal = Int (eval eRhs env) in
        let bodyEnv = (x, xVal) :: env in
            eval letBody bodyEnv
    | If (e1, e2, e3) ->
        let b = eval e1 env in
            if b <> 0 then
              eval e2 env
            else
              eval e3 env
    | Fun (f, x, fBody, letBody) ->
        let bodyEnv = (f, Closure (f, x, fBody, env)) :: env in
            eval letBody bodyEnv
    | Call (Var f, eArg) ->
        let fClosure = Env.lookup env f in
            begin
              match fClosure with
              | Closure (f, x, fBody, fDeclEnv) ->
                  let xVal = Int (eval eArg env) in
                  let fBodyEnv = (x, xVal) :: (f, fClosure) :: fDeclEnv in
                      eval fBody fBodyEnv
              | _ -> failwith "eval Call: not a function"
            end
    | Call _ -> failwith "eval Call: not first-order function"
