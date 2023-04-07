type FnResult = Result<f64, String>;

pub fn sqrt(n: f64) -> FnResult {
    if n < 0.0 {
        return Err("No se puede calcular la raíz cuadrada de un número negativo".to_string());
    }
    Ok(n.sqrt())
}

pub fn pow(a: f64, n: f64) -> f64 {
    a.powf(n)
}

pub fn factorial(n: f64) -> Result<f64, String> {
    if n.fract() != 0.0 {
        return Err("No se puede calcular el factorial de un real".to_string());
    }

    if n.is_sign_negative() {
        return Err("No se puede calcular el factorial de un número negativo".to_string());
    }

    let n = n as u64;
    let mut result = 1_u64;
    for i in 2..=n {
        result *= i;
    }
    Ok(result as f64)
}

pub fn sin(n: f64) -> f64 {
    n.sin()
}

pub fn cos(n: f64) -> f64 {
    n.cos()
}

pub fn tan(n: f64) -> f64 {
    n.tan()
}
