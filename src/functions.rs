// Aquí se definen múltiples funciones numéricas.
// Todas pueden recibir un número real o una matriz, y se validará correspondientemente.

use super::matrix::Matrix;
use super::value::Value;
use super::utils::nearly_equal;

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

    // Matriz cuadrada

    if a.is_square() {
        if !nearly_equal(a.determinant()?, 0.0) {
            // A es No singular ( invertible ) el sistema es compatible determinado
            // Ax=b
            // x=A^(-1)b
            return Ok(Value::Matrix(Matrix::multiply(&a.inverse()?, b)?)); 
        } else {
            // Matriz cuadrada
            // Determinante 0 ( singular )
            // Puede ser compatible indeterminado o incompatible

            let mut copya = a.clone();
            let mut copyb = b.clone();

            let n = copya.rows();

            for k in 0..n {
                // Obtengo el elemento de la diagonal (Akk, que será el pivote)
                let mut pivot = copya.get(k, k).unwrap();
                if nearly_equal(pivot, 0.0) {
                    // Busco la primera fila tal que Aik != 0
                    let mut found = false;
                    let mut i = k + 1;
                    while !found && i < n {
                        pivot = copya.get(i, k).unwrap();
                        if nearly_equal(pivot, 0.0) {
                            i += 1;
                        } else {
                            found = true;
                        }
                    }
                    if !found {
                        // No existe tal fila, el determinante es 0
                        // si k es la ultima fila significa que la ultima fila es todo 0
                        // si b[k][0] es 0 el sistema es indeterminado, sino incompatible
                        // en caso de que no se este evaluando la ultima fila se encontro una columna con todo ceros, que puede ser que defina algo, pero no sabria muy bien como chequearlo
                        if k == copya.rows() - 1 {
                            if copyb.get(k, 0)? == 0.0 {
                                return Err("[+] Sistema Compatible Indeterminado".to_string());
                            } else {
                                return Err("[+] Sistema Incompatible".to_string());
                            }
                        }
                        continue; // no hago cuanta del triangulito porque ya toda la columna es 0 o termino la matriz
                    } else {
                        // Intercambio la fila k con la fila i, intercambiando
                        // el valor de cada fila columna por columna.
                        for j in 0..n {
                            let tmp = copya.get(k, j)?;
                            copya.set(k, j, copya.get(i, j)?)?;
                            copya.set(i, j, tmp)?;
                        }
                        let tmp = copyb.get(k, 0)?;
                        copyb.set(k, 0, copyb.get(i, 0)?)?;
                        copyb.set(i, 0, tmp)?;
                    }
                }

                // Ahora, toca restar a cada fila i > k la fila k multiplicada por Aik/Akk

                // Itero fila por fila
                for i in (k + 1)..n {
                    // factor = Aik / Akk
                    let factor = copya.get(i, k)? / pivot;

                    // Resto a cada elemento de la fila i la fila k multiplicada por el factor
                    // Nótese que itero sobre las columnas k a n-1, ya que las columnas anteriores
                    // ya están en 0.
                    for j in k..n {
                        let new_value = copya.get(i, j)? - factor * copya.get(k, j)?;
                        copya.set(i, j, new_value)?;
                    }
                    let new_value = copyb.get(i, 0)? - factor * copyb.get(k, 0)?;
                    copyb.set(i, 0, new_value)?;
                }
            }
        }
    } else {
        if a.rows() < a.cols() {
            // Sistema Subdeterminado
            // Solo puede ser Compatible Indeterminado o Incompatible
            // println!("Estudio un sistema Subdeterminado M<N");

            let mut copya = a.clone();
            let mut copyb = b.clone();

            let n = copya.rows();

            for k in 0..n {
                // Obtengo el elemento de la diagonal (Akk, que será el pivote)
                let mut pivot = copya.get(k, k).unwrap();
                if nearly_equal(pivot, 0.0) {
                    // Busco la primera fila tal que Aik != 0
                    let mut found = false;
                    let mut i = k + 1;
                    while !found && i < n {
                        pivot = copya.get(i, k).unwrap();
                        if nearly_equal(pivot, 0.0) {
                            i += 1;
                        } else {
                            found = true;
                        }
                    }
                    if !found {
                        // No existe tal fila, el determinante es 0
                        // println!("Alguna fila es todo 0");
                        // println!("fila {} {}", k, copya);
                        // si k es la ultima fila significa que la ultima fila es todo 0
                        // si b[k][0] es 0 el sistema es indeterminado, sino incompatible
                        // en caso de que no se este evaluando la ultima fila se encontro una columna con todo ceros, que puede ser que defina algo, pero no sabria muy bien como chequearlo
                        if k == copya.rows() - 1 {
                            if copyb.get(k, 0)? == 0.0 {
                                return Err("[+] Sistema Compatible Indeterminado".to_string());
                            } else {
                                return Err("[+] Sistema Incompatible".to_string());
                            }
                        }
                        continue; // no hago cuanta del triangulito porque ya toda la columna es 0 o termino la matriz
                    } else {
                        // Intercambio la fila k con la fila i, intercambiando
                        // el valor de cada fila columna por columna.
                        for j in 0..copya.rows() {
                            let tmp = copya.get(k, j)?;
                            copya.set(k, j, copya.get(i, j)?)?;
                            copya.set(i, j, tmp)?;
                        }
                        let tmp = copyb.get(k, 0)?;
                        copyb.set(k, 0, copyb.get(i, 0)?)?;
                        copyb.set(i, 0, tmp)?;
                    }
                }

                // Ahora, toca restar a cada fila i > k la fila k multiplicada por Aik/Akk

                // Itero fila por fila
                for i in (k + 1)..n {
                    // factor = Aik / Akk
                    let factor = copya.get(i, k)? / pivot;

                    // Resto a cada elemento de la fila i la fila k multiplicada por el factor
                    // Nótese que itero sobre las columnas k a n-1, ya que las columnas anteriores
                    // ya están en 0.
                    for j in 0..copya.cols() {
                        let new_value = copya.get(i, j)? - factor * copya.get(k, j)?;
                        copya.set(i, j, new_value)?;
                    }
                    let new_value = copyb.get(i, 0)? - factor * copyb.get(k, 0)?;
                    copyb.set(i, 0, new_value)?;
                }
            }
            // println!("A:{}", copya);
            // println!("B:{}", copyb);

            // Solo llega a este punto si queda una matriz tal que (ejemplo con 2x3)
            // a b c | d
            // 0 b c | 0

            return Err("[+] Sistema Compatible Indeterminado".to_string());
        } else {
            // Sistema Subdeterminado\

            return Err("[!] Sistema Subdeterminado, esta funcion no resuelve bien los sistemas subdeterminados".to_string());

            // Puede ser Compatible Determinado, Compatible Indeterminado o Incompatible
            // println!("Estudio un sistema Sobredeterminado M>N");

            let mut copya = a.clone();
            let mut copyb = b.clone();

            let cols = copya.cols();
            let rows = copya.rows();

            for k in 0..cols {
                // Obtengo el elemento de la diagonal (Akk, que será el pivote)
                let mut pivot = copya.get(k, k).unwrap();
                if nearly_equal(pivot, 0.0) {
                    // Busco la primera fila tal que Aik != 0
                    let mut found = false;
                    let mut i = k + 1;
                    while !found && i < cols {
                        pivot = copya.get(i, k).unwrap();
                        if nearly_equal(pivot, 0.0) {
                            i += 1;
                        } else {
                            found = true;
                        }
                    }
                    if !found {
                        // si k es la ultima fila significa que la ultima fila es todo 0
                        // si b[k][0] es 0 el sistema es indeterminado, sino incompatible
                        // en caso de que no se este evaluando la ultima fila se encontro una columna con todo ceros, que puede ser que defina algo, pero no sabria muy bien como chequearlo
                        if k == copya.cols() - 1 {
                            // chequeo que las relaciones de la utima columna se mantengan, en caso de que no el sistema es incompatible. para que te hagas una idea 
                            // /  x=2
                            // \ 3x=3
                            // claramente este sistema es incompatible porque x tiene que ser 2 y 1 a la vez
                            let relation_one = copya.get(k,cols-1)? / copyb.get(k, 0)?;
                            let mut relation_equals = false;
                            for i in k..copya.rows() {
                                let value_incognita = copya.get(i, cols-1)?;
                                let value_b = copyb.get(i,0)?;
                                let relation_two = value_incognita / value_b;
                                
                                relation_equals = nearly_equal(relation_one, relation_two);

                                if !relation_equals {
                                    break
                                }

                            }
                            if relation_equals {
                                return Err("[+] Sistema Compatible Indeterminado".to_string())
                            } else {
                                return Err("[+] Sistema Incompatible".to_string())
                            }
                        }
                        continue; // no hago cuanta del triangulito porque ya toda la columna es 0 o termino la matriz
                    } else {
                        // Intercambio la fila k con la fila i, intercambiando
                        // el valor de cada fila columna por columna.
                        for j in 0..cols {
                            let tmp = copya.get(k, j)?;
                            copya.set(k, j, copya.get(i, j)?)?;
                            copya.set(i, j, tmp)?;
                        }
                        let tmp = copyb.get(k, 0)?;
                        copyb.set(k, 0, copyb.get(i, 0)?)?;
                        copyb.set(i, 0, tmp)?;
                    }
                }

                // Ahora, toca restar a cada fila i > k la fila k multiplicada por Aik/Akk

                // Itero fila por fila
                for i in k + 1..rows {
                    // factor = Aik / Akk
                    let factor = copya.get(i, k)? / pivot;

                    // Resto a cada elemento de la fila i la fila k multiplicada por el factor
                    // Nótese que itero sobre las columnas k a n-1, ya que las columnas anteriores
                    // ya están en 0.
                    for j in 0..cols {
                        let new_value = copya.get(i, j)? - factor * copya.get(k, j)?;
                        copya.set(i, j, new_value)?;
                    }
                    let new_value = copyb.get(i, 0)? - factor * copyb.get(k, 0)?;
                    copyb.set(i, 0, new_value)?;
                }
            }

            // println!("Fin Sobredeterminado");

            // println!("A:{}", copya);
            // println!("B:{}", copyb);

            // Solo llega a este punto si el sistema es compatible determinado o incompatible

            return Err("[+] Sistema Compatible Determinado (Por puro descarte)".to_string());
        }
    }

    Err("[+] Funcion Finalizada.".to_string())



        } else {
            Err("b debe ser una matriz.".to_string())
        }
    } else {
        Err("A debe ser una matriz".to_string())
    }
}
