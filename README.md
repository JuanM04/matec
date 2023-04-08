# Operaciones de Matrices

## Funcionameinto básico

```
> 1 + 1
ans = 2

> 2*6 ; ans/3 ; 2^ans
ans = 16

> t = sqrt(2)
t = 1.4142

> t^2
ans = 2

> [1,2,3]
ans =   1   2   3

> [1;2;3]
ans =   1
        2
        3

> [1,2,3;4,5,6;7,8,9]
ans =   1   2   3
        4   5   6
        7   8   9

> [1,2;1,3]\[4;5]
ans =   2
        1
```

### Operaciones

| Nombre | Descripción                         | Implementada |
| :----- | :---------------------------------- | :----------: |
| `+`    | Suma                                |      ✔️      |
| `-`    | Resta                               |      ✔️      |
| `*`    | Multiplicación                      |      ✔️      |
| `/`    | División                            |      ✔️      |
| `\`    | División a la derecha (`a/b = b\a`) |      ✔️      |
| `^`    | Potenciación                        |      ✔️      |
| `!`    | Factorial                           |      ✔️      |
| `'`    | Traspuesta (`A' = transpose(A)`)    |      ✔️      |

### Funciones

| Nombre           | Descripción                    | Implementada |
| :--------------- | :----------------------------- | :----------: |
| `exit`           | Termina el programa            |      ✔️      |
| `clc`            | Limpia la consola              |      ✔️      |
| `abs(n)`         | Valor absoluto de "n"          |      ✔️      |
| `sqrt(n)`        | Raíz cuadrada de "n"           |      ✔️      |
| `pow(a, n)`      | "a" a la n-ésima potencia      |      ✔️      |
| `inv(x)`         | Inverso de "x" (real o matriz) |      ✔️      |
| `factorial(n)`   | "n" factorial                  |      ✔️      |
| `sin(x)`         | Seno de "x"                    |      ✔️      |
| `cos(x)`         | Coseno de "x"                  |      ✔️      |
| `tan(x)`         | Tangente de "x"                |      ✔️      |
| `log(x)`         | Logarítmo natural de "x"       |      ✔️      |
| `transpose(A)`   | Transpuesta de la matriz "A"   |      ✔️      |
| `det(A)`         | Determinante de la matriz "A"  |      ✔️      |
| `linsolve(A, b)` | Resuelve un sistema lineal     |      ❌      |

### Variables

| Nombre | Valor                            | Implementada |
| :----: | :------------------------------- | :----------: |
|  ans   | Resultado de la última operación |      ✔️      |
|   pi   | 3.1415 ...                       |      ✔️      |
|   e    | 2.7182 ...                       |      ✔️      |
