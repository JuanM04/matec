// Aquí se definen múltiples funciones numéricas.
// Todas pueden recibir un número real o una matriz, y se validará correspondientemente.

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

// Calcula el inverso de un valor.
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

/// Resuelve un sistema de ecuaciones lineales.
pub fn linsolve(a: &Value, b: &Value) -> FnResult {
    if let Value::Matrix(a) = a {
        if let Value::Matrix(b) = b {
            // En linsolve se puede hacer que si b es un scalar hacer una matriz donde todos sus indices sean ese escalar ( mas que nada implementarlo para 0 por los sistemas homogeneos)
            // Evaluar que a y b tengan la misma cantidad de filas
            // Revisar que a sea cuadrada
            // Revisar que b tenga una sola columna

            if a.rows() != b.rows() {
                return Err("La cantidad de filas de A y b no coincide".to_string());
            }

            if b.cols() != 1 {
                return Err("La matriz b debe tener una sola columna".to_string());
            }

            if a.is_square() && !nearly_equal(a.determinant()?, 0.0) {
                // A es No singular ( invertible ) el sistema es compatible determinado
                // Ax=b
                // x=A^(-1)b
                return Ok(Value::Matrix(Matrix::multiply(&a.inverse()?, b)?));
            }

            // Creo la matriz ecuacion (A|b)

            // le sumo 1 a cols por el vector B
            let rows = a.rows();
            let cols = a.cols() + 1;
            let mut matrix = Matrix::new(rows, cols);

            // asigno los valores de A en la matriz
            for (row, col, val) in a {
                matrix.set(row, col, val)?;
            }

            // asigno los valores de b en la matriz
            for i in 0..rows {
                matrix.set(i, cols - 1, b.get(i, 0)?)?
            }

            let mut k = 0;
            while k < rows && k < cols {
                // Obtengo el elemento de la diagonal (Akk, que será el pivote)
                let mut pivot = matrix.get(k, k).unwrap();
                if nearly_equal(pivot, 0.0) {
                    // Busco la primera fila tal que Aik != 0
                    let mut found = false;
                    // Solo busco en las filas k+1 a n-1, ya que las filas anteriores ya están en 0
                    let mut i = k + 1;
                    while !found && i < rows {
                        pivot = matrix.get(i, k).unwrap();
                        if nearly_equal(pivot, 0.0) {
                            i += 1;
                        } else {
                            found = true;
                        }
                    }
                    if !found {
                        // Nota: este mensaje no se debería mostrar nunca, ya que el determinante
                        // debería ser 0. Como nadie quiere un bucle infinito, lo dejo por las dudas.
                        k += 1;
                        continue;
                    } else {
                        for j in 0..cols {
                            let tmp = matrix.get(k, j)?;
                            matrix.set(k, j, matrix.get(i, j)?)?;
                            matrix.set(i, j, tmp)?;
                        }
                    }
                }

                // Ahora, toca dividir cada elemento de la fila k por Akk.
                // Creo la matriz elemental que divide la fila k por Akk
                let scale = 1.0 / matrix.get(k, k)?;

                for j in 0..cols {
                    matrix.set(k, j, matrix.get(k, j)? * scale)?;
                }

                // Ahora, toca restar a cada fila i != k la fila k multiplicada por Aik.
                let mut i = 0;
                while i < rows && i < cols {
                    if i != k {
                        let factor = matrix.get(i, k)? / matrix.get(k, k).unwrap();

                        for j in 0..cols {
                            let new_value = matrix.get(i, j)? - factor * matrix.get(k, j)?;
                            matrix.set(i, j, new_value)?;
                        }
                    }
                    i += 1;
                }
                k += 1;
            }

            // Reviso que la diagonal sean todos numeros distintos de cero
            let mut k = 0;
            let mut diagonal_not_cero = true;
            while k < rows && k < cols - 1 {
                // si la diagonal tiene un cero diagonal not cero = false
                if nearly_equal(matrix.get(k, k)?, 0.0) {
                    diagonal_not_cero = false;
                    break;
                }
                k += 1;
            }

            // Miro que las ultima fila de A sea de ceros
            let mut row_all_ceros = true;
            let mut index_col_cero: i32 = -1;
            let mut index_row_cero: i32 = -1;
            for k in (0..rows).rev() {
                for i in 0..cols {
                    // miro las filas de abajo para arriba, chequeo que toda la fila sea 0
                    // en caso de que no sea 0 reviso que el indice encontrado sea de la matriz A o de b
                    // si el indice esta en la matriz A forma n sistema compatile indeterminado.
                    // si le indice esta en la matriz b forma un sistema incompatible ya que no hay valor
                    //   que puedan tomar las incognitas para igualar este valor (todas estan multiplicadas por cero)
                    if matrix.get(k, i)? != 0.0 {
                        row_all_ceros = false;
                        index_row_cero = k as i32;
                        index_col_cero = i as i32;
                        break;
                    }
                }
                if !row_all_ceros {
                    break;
                }
            }

            if !row_all_ceros {
                if index_col_cero == (cols - 1) as i32 {
                    println!("[!] Sistema Incompatible");
                    return Ok(Value::Scalar(0.0));
                } else if !(index_row_cero == (cols - 2) as i32 && diagonal_not_cero) {
                    println!("[!] Sistema Compatible Indeterminado");
                    return Ok(Value::Scalar(0.0));
                }
            }

            // creo matriz CS de sistema determinado

            let mut result = Matrix::new(a.cols(),1);
            for i in 0..a.cols() {
                result.set(i, 0, matrix.get(i, cols -1)?)?
            }

            return Ok(Value::Matrix(result));

        } else {
            Err("[!] b debe ser una matriz.".to_string())
        }
    } else {
        Err("[!] A debe ser una matriz".to_string())
    }
}
