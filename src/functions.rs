// Aquí se definen múltiples funciones numéricas.
// Todas pueden recibir un número real o una matriz, y se validará correspondientemente.

use super::value::Value;

type FnResult = Result<Value, String>;

/// Suma dos valores.
pub fn add(left: &Value, right: &Value) -> FnResult {
    match (left, right) {
        // Si ambos son números reales, se suman.
        (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a + b)),
        // Si ambos son matrices, se suman.
        // Ver cómo se implementa la suma de matrices en matrix/mod.rs
        (Value::Matrix(a), Value::Matrix(b)) => Ok(Value::Matrix(a.add(b)?)),
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
        (Value::Matrix(a), Value::Matrix(b)) => Ok(Value::Matrix(a.mul(b)?)),
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
            if *x == 0.0 {
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
    unimplemented!()
}
