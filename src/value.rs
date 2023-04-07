use super::matrix::Matrix;
use std::fmt;

#[derive(Clone)]
pub enum Value {
    Scalar(f64),
    Matrix(Matrix),
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
          Value::Scalar(s) => write!(f, "{}", s),
          Value::Matrix(m) => write!(f, "{}", m),
      }
  }
}
