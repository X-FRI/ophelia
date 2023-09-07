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
    | i = INT { CstI i }
    | x = ID { Var x }
    | TRUE { CstB true }
    | FALSE { CstB false }
    | IF; e1 = expr; THEN; e2 = expr; ELSE; e3 = expr { If (e1, e2, e3) }
    | LET; x = ID; EQUALS; e1 = expr; IN; e2 = expr { Let (x, e1, e2) }
    | FUN; f = ID; x = ID; RIGHT_ALLOW; e1 = expr; IN; e2 = expr { Fun (f, x, e1, e2) }
    | f = ID; LPAREN; e = expr; RPAREN { Call(Var f, e) }
    | e1 = expr; MUL; e2 = expr { Prim ("*", e1, e2) }
    | e1 = expr; EQUALS; e2 = expr { Prim ("=", e1, e2) }
    | e1 = expr; PLUS; e2 = expr { Prim ("+", e1, e2) }
    | e1 = expr; SUB; e2 = expr { Prim ("-", e1, e2) }
    | e1 = expr; DIV; e2 = expr { Prim ("/", e1, e2) }
;;