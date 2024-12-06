本项目为个人对PKU编译原理项目的实践 @see https://pku-minic.github.io/online-doc/#/



## SysY规范

SysY是C语言的子集,一门图灵完备的语言

本项目支持的语法如下:

```ebnf
CompUnit      ::= [CompUnit] (Decl | FuncDef);

Decl          ::= ConstDecl | VarDecl;
ConstDecl     ::= "const" BType ConstDef {"," ConstDef} ";";
BType         ::= "int";
ConstDef      ::= IDENT {"[" ConstExp "]"} "=" ConstInitVal;
ConstInitVal  ::= ConstExp | "{" [ConstInitVal {"," ConstInitVal}] "}";
VarDecl       ::= BType VarDef {"," VarDef} ";";
VarDef        ::= IDENT {"[" ConstExp "]"}
                | IDENT {"[" ConstExp "]"} "=" InitVal;
InitVal       ::= Exp | "{" [InitVal {"," InitVal}] "}";

FuncDef       ::= FuncType IDENT "(" [FuncFParams] ")" Block;
FuncType      ::= "void" | "int";
FuncFParams   ::= FuncFParam {"," FuncFParam};
FuncFParam    ::= BType IDENT ["[" "]" {"[" ConstExp "]"}];

Block         ::= "{" {BlockItem} "}";
BlockItem     ::= Decl | Stmt;
Stmt          ::= LVal "=" Exp ";"
                | [Exp] ";"
                | Block
                | "if" "(" Exp ")" Stmt ["else" Stmt]
                | "while" "(" Exp ")" Stmt
                | "break" ";"
                | "continue" ";"
                | "return" [Exp] ";";

Exp           ::= LOrExp;
LVal          ::= IDENT {"[" Exp "]"};
PrimaryExp    ::= "(" Exp ")" | LVal | Number;
Number        ::= INT_CONST;
UnaryExp      ::= PrimaryExp | IDENT "(" [FuncRParams] ")" | UnaryOp UnaryExp;
UnaryOp       ::= "+" | "-" | "!";
FuncRParams   ::= Exp {"," Exp};
MulExp        ::= UnaryExp | MulExp ("*" | "/" | "%") UnaryExp;
AddExp        ::= MulExp | AddExp ("+" | "-") MulExp;
RelExp        ::= AddExp | RelExp ("<" | ">" | "<=" | ">=") AddExp;
EqExp         ::= RelExp | EqExp ("==" | "!=") RelExp;
LAndExp       ::= EqExp | LAndExp "&&" EqExp;
LOrExp        ::= LAndExp | LOrExp "||" LAndExp;
ConstExp      ::= Exp;
```



一个例程：

~~~C
int buf[2][100];

// sort [l, r)
void merge_sort(int l, int r)
{
    if (l + 1 >= r)
        return;

    int mid = (l + r) / 2;
    merge_sort(l, mid);
    merge_sort(mid, r);

    int i = l, j = mid, k = l;
    while (i < mid && j < r) {
        if (buf[0][i] < buf[0][j]) {
            buf[1][k] = buf[0][i];
            i = i + 1;
        } else {
            buf[1][k] = buf[0][j];
            j = j + 1;
        }
        k = k + 1;
    }
    while (i < mid) {
        buf[1][k] = buf[0][i];
        i = i + 1;
        k = k + 1;
    }
    while (j < r) {
        buf[1][k] = buf[0][j];
        j = j + 1;
        k = k + 1;
    }

    while (l < r) {
        buf[0][l] = buf[1][l];
        l = l + 1;
    }
}

int main()
{
    int n = getarray(buf[0]);
    merge_sort(0, n);
    putarray(n, buf[0]);
    return 0;
}
~~~



## 测试

https://github.com/pku-minic/compiler-dev-test-cases/tree/master

通过以上链接中所有程序从C到Koopa,C到RISCV的测试



## 待进行的优化

mark一下后续的优化计划

1.尾递归

2.寄存器分配



## 写到后面

感谢 Maxxing 的无私奉献
