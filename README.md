<div align="center">

# Ophelia

A minimalist imperative language compiled to RISC-V for [OpheliaOS](https://github.com/muqiuhan/OpheliaOS)

![](https://github.com/muqiuhan/ophelia/actions/workflows/ci.yaml/badge.svg) 

🚧 Working In Progress.

</div>

## Syntax
```ebnf
# identifier-nondigit is the underline, lowercase letters or uppercase letters
# digit is 0~9
identifier ::= identifier-nondigit
             | identifier identifier-nondigit
             | identifier digit;

# nonzero-digit is 1~9
# octal-digit is 0~7
# hexadecimal-digit is 0~9, or (lower/upper)case letter A~f
integer-const ::= decimal-const
                  | octal-const
                  | hexadecimal-const;

decimal-const ::= nonzero-digit
                  | decimal-const digit;

octal-const ::= "0"
                | octal-const octal-digit;

hexadecimal-const ::= hexadecimal-prefix hexadecimal-digit
                      | hexadecimal-const hexadecimal-digit;

hexadecimal-prefix ::= "0x" | "0X";

comment ::= single-line-comment
single-line-comment ::= "#"

CompUnit  ::= FuncDef;

FuncDef   ::= FuncType IDENT "(" ")" Block;
FuncType  ::= "int";

Block     ::= "{" Stmt "}";
Stmt      ::= "return" Number ";";

# The range of INT_CONST is 0~(2^31-1) (no negative sign).
Number    ::= INT_CONST;
```

## LICENSE
The MIT License (MIT)

Copyright (c) 2023 Muqiu Han

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.