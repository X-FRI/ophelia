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

%{
    open Ast
%}

%token <int> INT
%token <string> ID
%token TRUE
%token FALSE
%token RIGHT_ALLOW
%token MUL
%token SUB
%token DIV
%token PLUS
%token LPAREN
%token RPAREN
%token LET
%token EQUALS
%token IN
%token IF
%token THEN
%token ELSE
%token EOF
%token FUN
%token LEFT_CURLY
%token RIGHT_CURLY

%nonassoc IN
%nonassoc ELSE
%left PLUS
%left MUL
%left RIGHT_ALLOW

%start <Ast.expr> prog

%%

prog:
    | e = expr; EOF { e }
;;

expr:
    | i = INT { Literal(Literal_int i) }
    | x = ID { Identifier x }
    | TRUE { Literal(Literal_bool true) }
    | FALSE { Literal(Literal_bool false) }
    | IF; e1 = expr; THEN; e2 = expr; ELSE; e3 = expr { If (e1, e2, e3) }
    | LET; x = ID; EQUALS; e1 = expr; IN; e2 = expr { Define(Define_var(x, e1, e2)) }
    | FUN; f = ID; x = ID; RIGHT_ALLOW; e1 = expr; IN; e2 = expr { Define(Define_fun (f, x, e1, e2)) }
    | f = ID; LPAREN; e = expr; RPAREN { Call(Identifier f, e) }
    | e1 = expr; MUL; e2 = expr { Prim ("*", e1, e2) }
    | e1 = expr; EQUALS; e2 = expr { Prim ("=", e1, e2) }
    | e1 = expr; PLUS; e2 = expr { Prim ("+", e1, e2) }
    | e1 = expr; SUB; e2 = expr { Prim ("-", e1, e2) }
    | e1 = expr; DIV; e2 = expr { Prim ("/", e1, e2) }
;;