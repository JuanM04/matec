# Operaciones de Matrices

Trabajo práctico para Matemática C, 2023.

Descargas para [Windows (32 bits)](https://github.com/JuanM04/matec/releases/download/v1.0.0/matrices-1.0.0-windows-x86.exe), [Windows (64 bits)](https://github.com/JuanM04/matec/releases/download/v1.0.0/matrices-1.0.0-windows-x64.exe) y [Linux](https://github.com/JuanM04/matec/releases/download/v1.0.0/matrices-1.0.0-linux.tar.gz).

## Desarrollo

Se necesita tener [Rust](https://www.rust-lang.org/es/) instalado. Luego, para correr el código basta con ejecutar `cargo run`. Para compilarlo, se corre `cargo build --release` y el ejecutable se encontrará en `target/release/matec`.

> **Note**  
> Se recomienda usar [Visual Studio Code](https://code.visualstudio.com/) junto a las extensiones recomendadas en `.vscode/extensions.json`.

## Especificación

### Operaciones

| Nombre              | Descripción                              |
| :------------------ | :--------------------------------------- |
| `+`                 | Suma                                     |
| `-`                 | Resta                                    |
| `*`                 | Multiplicación                           |
| `/`                 | División                                 |
| `\`                 | División a la derecha (`a/b = b\a`)      |
| `^`, `pow(a, n)`    | Potenciación                             |
| `!`, `factorial(n)` | Factorial                                |
| `'`, `transpose(A)` | Traspuesta de una matriz                 |
| `abs(n)`            | Valor absoluto                           |
| `sqrt(n)`           | Raíz cuadrada                            |
| `inv(a)`            | Inverso (de un real o de una matriz)     |
| `sin(x)`            | Seno                                     |
| `cos(x)`            | Coseno                                   |
| `tan(x)`            | Tangente                                 |
| `log(x)`            | Logarítmo natural                        |
| `det(A)`            | Determinante                             |
| `linsolve(A, b)`    | Resuelve un sistema de ecuaciones lineal |

### Comandos

| Nombre | Descripción                  |
| :----- | :------------------          |
| `help` | Mostrar comandos disponibles |
| `exit` | Termina el programa          |
| `clc`  | Limpia la consola            |

### Variables

| Nombre | Valor                            |
| :----: | :------------------------------- |
|  ans   | Resultado de la última operación |
|   pi   | 3.1415 ...                       |
|   e    | 2.7182 ...                       |
