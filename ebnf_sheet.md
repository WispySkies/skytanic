## Simple EBNF For Spreadsheets

```
Expression -> Equation | Literal

Equation -> '=' BinaryExpression

BinaryExpression -> BinaryExpression BinOP BinaryExpression
                  | UnaryExpression

UnaryExpression -> UnaryOp Primary
                 | Primary

Primary -> Literal
         | CellReference
         | FunctionCall
         | '(' BinaryExpression ')'

Literal -> INT | FLOAT | BOOL | STRING

CellReference -> STRING INT

BinOP -> '+' | '-' | '*' | '/' | '%' | '**'
       | '&&' | '||' | '&' | '|' | '^'
       | '<=' | '<' | '==' | '!=' | '>'
       | '>=' | '<<' | '>>'

UnaryOp -> '!' | '~' | '-'
```