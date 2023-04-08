use super::matrix::Matrix;
use super::utils::format_float;
use std::fmt;

#[derive(Clone)]
pub enum Value {
    Scalar(f64),
    Matrix(Matrix),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Scalar(s) => write!(f, "{}", format_float(*s)),
            Value::Matrix(m) => write!(f, "{}", m),
        }
    }
}
