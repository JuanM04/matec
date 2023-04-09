// Aquí se definen múltiples funciones numéricas.
// Todas pueden recibir un número real o una matriz, y se validará correspondientemente.

use crate::utils::format_float;

use super::matrix::Matrix;
use super::utils::nearly_equal;
use super::value::Value;

type FnResult = Result<Value, String>;

/// Suma dos valores.
pub fn add(left: &Value, right: &Value) -> FnResult {
    match (left, right) {
        // Si ambos son números reales, se suman.
        (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a + b)),
        // Si ambos son matrices, se suman.
        // Ver cómo se implementa la suma de matrices en matrix/mod.rs
        (Value::Matrix(a), Value::Matrix(b)) => Ok(Value::Matrix(Matrix::add(a, b)?)),
        _ => Err("La suma entre matrices y reales no está definida".to_string()),
    }
}

/// Calcula el opuesto de un valor.
pub fn negate(x: &Value) -> FnResult {
    match x {
        // Si es un número real, se multiplica por -1.
        Value::Scalar(x) => Ok(Value::Scalar(-x)),
        // Si es una matriz, se lo escala por -1.
        // Ver cómo se implementa la multiplicación por un escalar en matrix/mod.rs
        Value::Matrix(a) => Ok(Value::Matrix(a.scale(-1.0))),
    }
}

/// Resta dos valores.
pub fn subtract(left: &Value, right: &Value) -> FnResult {
    // Se ejecuta la suma de a y el opuesto de b.
    add(left, &negate(right)?)
}

/// Multiplica dos valores.
pub fn multiply(left: &Value, right: &Value) -> FnResult {
    match (left, right) {
        // Si ambos son números reales, se multiplican.
        (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a * b)),
        // Si ambos son matrices, se multiplican.
        // Ver cómo se implementa la multiplicación de matrices en matrix/mod.rs
        (Value::Matrix(a), Value::Matrix(b)) => Ok(Value::Matrix(Matrix::multiply(a, b)?)),
        // Si uno es un número real y el otro una matriz, se escala la matriz por el número.
        // Ver cómo se implementa la multiplicación por un escalar en matrix/mod.rs
        (Value::Scalar(a), Value::Matrix(b)) => Ok(Value::Matrix(b.scale(*a))),
        (Value::Matrix(a), Value::Scalar(b)) => Ok(Value::Matrix(a.scale(*b))),
    }
}

/// Calcula el inverso de un valor.
pub fn inverse(x: &Value) -> FnResult {
    match x {
        // Si es un número real, se divide 1 entre él.
        Value::Scalar(x) => {
            if nearly_equal(*x, 0.0) {
                return Err("1/0 no está definido".to_string());
            }
            Ok(Value::Scalar(1.0 / x))
        }
        // Si es una matriz, se calcula su inversa.
        // Ver cómo se implementa la inversa de matrices en matrix/mod.rs
        Value::Matrix(a) => Ok(Value::Matrix(a.inverse()?)),
    }
}

/// Divide dos valores.
pub fn divide(left: &Value, right: &Value) -> FnResult {
    // Se ejecuta la multiplicación de a y el inverso de b.
    multiply(left, &inverse(right)?)
}

/// Divide dos valores (versión derecha).
pub fn right_divide(left: &Value, right: &Value) -> FnResult {
    // a/b = b\a. Se invierte el factor izquierdo y se multiplica por el derecho.
    multiply(&inverse(left)?, right)
}

/// Eleva un valor a una potencia.
pub fn pow(a: &Value, n: &Value) -> FnResult {
    if let Value::Scalar(n) = n {
        match a {
            // Si es un número real, se eleva a la potencia.
            Value::Scalar(a) => Ok(Value::Scalar(a.powf(*n))),
            // Si es una matriz, se eleva a la potencia.
            // Ver cómo se implementa la potencia de matrices en matrix/mod.rs
            Value::Matrix(a) => Ok(Value::Matrix(a.pow(*n)?)),
        }
    } else {
        Err("El exponente de la potencia no puede ser una matriz".to_string())
    }
}

/// Calcula el valor absoluto de un valor.
pub fn abs(n: &Value) -> FnResult {
    if let Value::Scalar(n) = n {
        Ok(Value::Scalar(n.abs()))
    } else {
        Err("sqrt() solo puede ser usada con números reales".to_string())
    }
}

/// Calcula la raíz cuadrada de un valor.
pub fn sqrt(n: &Value) -> FnResult {
    if let Value::Scalar(n) = n {
        if *n < 0.0 {
            return Err("No se puede calcular la raíz cuadrada de un número negativo".to_string());
        }
        Ok(Value::Scalar(n.sqrt()))
    } else {
        Err("sqrt() solo puede ser usada con números reales".to_string())
    }
}

/// Calcula el factorial de un valor.
pub fn factorial(n: &Value) -> FnResult {
    if let Value::Scalar(n) = n {
        if *n < 0.0 {
            return Err("No se puede calcular el factorial de un número negativo".to_string());
        }
        let n = *n as u64;
        let mut result = 1_u64;
        for i in 2..=n {
            result *= i;
        }
        Ok(Value::Scalar(result as f64))
    } else {
        Err("El factorial no está definido para matrices".to_string())
    }
}

/// Calcula el seno de un valor.
pub fn sin(x: &Value) -> FnResult {
    match x {
        Value::Scalar(x) => Ok(Value::Scalar(x.sin())),
        Value::Matrix(_) => Err("El seno no está definido para matrices".to_string()),
    }
}

/// Calcula el coseno de un valor.
pub fn cos(x: &Value) -> FnResult {
    match x {
        Value::Scalar(x) => Ok(Value::Scalar(x.cos())),
        Value::Matrix(_) => Err("El coseno no está definido para matrices".to_string()),
    }
}

/// Calcula la tangente de un valor.
pub fn tan(x: &Value) -> FnResult {
    match x {
        Value::Scalar(x) => Ok(Value::Scalar(x.tan())),
        Value::Matrix(_) => Err("La tangente no está definido para matrices".to_string()),
    }
}

/// Calcula el logarítmo natural de un valor.
pub fn log(x: &Value) -> FnResult {
    match x {
        Value::Scalar(x) => Ok(Value::Scalar(x.ln())),
        Value::Matrix(_) => Err("El logarítmo no está definido para matrices".to_string()),
    }
}

/// Calcula la traspuesta de una matriz.
pub fn transpose(a: &Value) -> FnResult {
    if let Value::Matrix(a) = a {
        // Ver cómo se implementa la traspuesta de una matriz en matrix/mod.rs
        Ok(Value::Matrix(a.transpose()))
    } else {
        Err("La traspuesta no está definida para números reales".to_string())
    }
}

/// Calcula el determinante de una matriz.
pub fn det(a: &Value) -> FnResult {
    if let Value::Matrix(a) = a {
        // Ver cómo se implementa el determinante de una matriz en matrix/mod.rs
        Ok(Value::Scalar(a.determinant()?))
    } else {
        Err("La traspuesta no está definida para números reales".to_string())
    }
}

/// Resuelve un sistema de ecuaciones lineales de la forma Ax = b.
/// A: matriz de coeficientes
/// b: vector columna de términos independientes
///
/// Se resuelve obteniendo la forma escalonada reducida de Gauss-Jordan.
pub fn linsolve(a: &Value, b: &Value) -> FnResult {
    if let Value::Matrix(a) = a {
        if let Value::Matrix(b) = b {
            if a.cols() == 0 {
                return Err("La matriz A no puede ser vacía".to_string());
            }

            if a.rows() != b.rows() {
                return Err("La cantidad de filas de A y b no coincide".to_string());
            }

            if b.cols() != 1 {
                return Err("La matriz b debe tener una sola columna".to_string());
            }

            let inverse = a.inverse();
            if let Ok(inverse) = inverse {
                // Si existe la inversa de A, A no es singular y, por ende,
                // el sistema es compatible determinado. x = A^(-1)b

                println!("El sistema es compatible determinado");
                return Ok(Value::Matrix(Matrix::multiply(&inverse, b)?));
            }

            let rows = a.rows();
            let cols = a.cols();
            // Creo la matriz aumentada (A|b)
            let mut matrix = Matrix::new(rows, cols + 1);

            // Copio los valores de A en la matriz
            for (row, col, val) in a {
                matrix.set(row, col, val)?;
            }

            // Copio los valores de b en la matriz
            for i in 0..rows {
                matrix.set(i, cols, b.get(i, 0)?)?
            }

            // Recorro la diagonal con un i y un j.
            // El i es el índice de la fila y el j el de la columna.
            //
            // La estrategia es buscar la primera fila tal que Akj != 0 e intercambiarla con la fila i.
            // Si no existe tal fila, se avanza a la columna siguiente y se repite el proceso.
            //
            // Una vez encontrado el pivote, se permuta y se divide cada elemento de la fila i por Aij.
            // Así, Aij = 1. Luego, se resta a cada fila k != i la fila i multiplicada por Akj. Así, los elementos
            // de la columna j quedan en 0 para esas filas.
            //
            // Todo esto para que quede la matriz en forma escalonada reducida de Gauss-Jordan, la cual
            // será analizada más adelante.

            let mut i: usize = 0;
            let mut j: usize = 0;
            while i < rows && j < cols {
                // Obtengo el elemento de la diagonal (Aij, que será el pivote)
                let mut pivot = matrix.get(i, j).unwrap();
                if nearly_equal(pivot, 0.0) {
                    // Busco la primera fila tal que Akj != 0
                    let mut found = false;
                    // Solo busco en las filas i+1 a rows-1, ya que las filas anteriores ya están en 0
                    let mut k = i + 1;
                    while !found && k < rows {
                        pivot = matrix.get(k, j).unwrap();
                        if nearly_equal(pivot, 0.0) {
                            k += 1;
                        } else {
                            found = true;
                        }
                    }
                    if !found {
                        // No encontré ningún elemento no nulo en la columna j
                        // Por lo tanto, paso a la siguiente columna
                        j += 1;
                        continue;
                    } else {
                        // Permuto la fila k con la fila i
                        matrix.swap_rows(k, i)?;
                    }
                }

                // Divido la fila i por Aij, así Aij = 1
                let factor = 1.0 / pivot;
                matrix.scale_row(i, factor)?;

                // Ahora, toca restar a cada fila k != i la fila i multiplicada por Akj.
                for k in 0..rows {
                    if k != i {
                        // factor = -Akj
                        let factor = -matrix.get(k, j)?;

                        // Sumo a cada elemento de la fila k la fila i multiplicada por el factor,
                        // así los elementos de la columna k quedan en 0.
                        matrix.add_row(k, i, factor)?;
                    }
                }

                // Avanzo en diagonal
                i += 1;
                j += 1;
            }

            // Ahora, la matriz está en forma escalonada reducida de Gauss-Jordan.

            // Por cómo se construyó la matriz, si existen filas nulas, estas serán las últimas.
            // Así, empezando desde la última fila, compruebo que todas las filas nulas sean de la forma
            // 0 ... 0 | b con b != 0. Si esto no se cumple, el sistema es incompatible.

            i = rows - 1;
            while i > 0 {
                let mut row_all_ceros = true;
                j = 0;
                while row_all_ceros && j < cols {
                    if !nearly_equal(matrix.get(i, j)?, 0.0) {
                        row_all_ceros = false;
                    }
                    j += 1;
                }

                if !row_all_ceros {
                    // La fila no es nula, por lo que ninguna de las filas restantes será nula.
                    // Por lo tanto, el sistema es compatible.
                    break;
                }

                // La fila es nula, por lo que compruebo que b != 0

                let b = matrix.get(i, cols)?;
                if !nearly_equal(b, 0.0) {
                    // La fila es nula y b != 0, por lo que el sistema es incompatible.
                    return Err("El sistema es incompatible".to_string());
                }

                i -= 1;
            }

            // Para este punto, el sistema es compatible.
            // Para saber si es determinado o indeterminado, nos podemos aprovechar del índice i
            // que nos indica la última fila no nula.
            //
            // Si i + 1 < cols, el sistema es indeterminado.
            // Si i + 1 = cols, el sistema es determinado.
            //
            // Por como se construyó la matriz, si la última fila no nula es la fila coincide con
            // la cantidad de variables, el sistema es determinado, ya que habrá quedado una diagonal
            // de unos y el resto de la matriz será 0.

            if i == cols - 1 {
                // El sistema es determinado

                // Obtengo la única solución del sistema, que es la última columna de la matriz.
                let mut solution = Matrix::new(cols, 1);
                for i in 0..cols {
                    solution.set(i, 0, matrix.get(i, cols)?)?;
                }

                println!("El sistema es compatible determinado");
                return Ok(Value::Matrix(solution));
            } else {
                // El sistema es indeterminado

                // Obtengo el vector solución.
                // Como hay variables independientes, este será un vector de Strings que será de la forma
                // x1 = 1
                // x2 = 5
                // x3 = 3 + 7*x4

                let mut vars = Vec::<String>::new();

                // Recorro la matriz y obtengo los valores de las variables
                i = 0;
                while i < rows {
                    let mut j: usize = 0;
                    let b = matrix.get(i, cols)?;

                    // Busco el primer elemento no nulo de la fila
                    while j < cols && nearly_equal(matrix.get(i, j)?, 0.0) {
                        j += 1;
                    }

                    if j == cols {
                        // Es una fila nula, por lo que ya no hay nada más que analizar.
                        break;
                    }

                    // La variable dependiente será xj (es decir, "x1", "x2", etc.)
                    let mut var = format!("x{} = {}", j + 1, format_float(b));

                    // Busco variables independientes
                    j += 1;
                    while j < cols {
                        let x = matrix.get(i, j)?;
                        if !nearly_equal(x, 0.0) {
                            // Si x != 0, entonces la variable independiente es xj

                            // Como el despeje que se hace es
                            // x + y = b => x = b - y, el signo se invierte
                            let sign = if x > 0.0 { " - " } else { " + " };
                            let factor = format_float(x.abs());
                            var.push_str(&format!(" {sign} {factor}*x{n}", n = (j + 1)));
                        }

                        j += 1;
                    }
                    vars.push(var);
                    i += 1;
                }

                println!("El sistema es compatible indeterminado. El conjunto solución es:\n");

                // Imprimo el conjunto solución
                for var in &vars {
                    println!("{}", var);
                }

                println!(
                    "\nEl sistema tiene {} variables dependientes y {} variables independientes\n",
                    vars.len(),
                    cols - vars.len(),
                );

                return Err("El sistema no tiene una única solución".to_string());
            }
        } else {
            Err("b debe ser una matriz.".to_string())
        }
    } else {
        Err("A debe ser una matriz".to_string())
    }
}
