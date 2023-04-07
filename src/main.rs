mod functions;
mod matrix;
mod parser;
mod value;

use functions::scalars;
use matrix::Matrix;
use parser::{parse, AstNode};
use std::io::{stdin, stdout, Write};
use value::Value;

fn main() {
    println!("#===========================#");
    println!("# Clon sin nombre de Matlab #");
    println!("#===========================#");
    println!("");
    println!("Por Majoros, Lorenzo; y Seery, Juan Martín");
    println!("Para Matemática C - 2023");
    println!("");
    println!("");
    println!("Para ayuda, escriba \"help\"");
    println!("Para salir, escriba \"exit\"");
    println!("");
    println!("");

    loop {
        print!("> ");
        let mut input = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let result = parse(&input);
        if let Ok(ast) = result {
            for n in &ast {
                match evaluate_expression(&n.expr) {
                    Ok(ans) => println!("ans = {}", ans),
                    Err(e) => println!("Error: {}", e),
                }
            }
        } else {
            println!("Error de sintáxis: {:#?}", result);
        }
    }
}

type ExprResult = Result<Value, String>;

fn evaluate_expression(expr: &AstNode) -> ExprResult {
    match expr {
        AstNode::Ident(s) => unimplemented!(),
        AstNode::Scalar(n) => Ok(Value::Scalar(*n)),
        AstNode::Matrix(vec) => {
            let rows = vec.len();
            if rows == 0 {
                return Ok(Value::Matrix(Matrix::new(0, 0)));
            }
            let cols = vec[0].len();
            let mut matrix = Matrix::new(rows, cols);
            for (i, row) in vec.iter().enumerate() {
                for (j, col) in row.iter().enumerate() {
                    if j + 1 > cols {
                        return Err(
                            "La matriz está mal declarada: el número de columnas no es consistente"
                                .to_string(),
                        );
                    }

                    match evaluate_expression(col) {
                        Ok(Value::Scalar(n)) => matrix.set(i, j, n).unwrap(),
                        Ok(Value::Matrix(_)) => {
                            return Err(
                                "No se puede declarar una matriz dentro de otra matriz".to_string()
                            )
                        }
                        Err(e) => return Err(e),
                    };
                }
            }
            Ok(Value::Matrix(matrix))
        }
        AstNode::Call { func, args } => unimplemented!(),
        AstNode::UnaryOp { op, expr } => {
            let value = evaluate_expression(expr)?;
            match op {
                parser::UnaryOp::Positive => Ok(value),
                parser::UnaryOp::Negate => match value {
                    Value::Scalar(n) => Ok(Value::Scalar(-n)),
                    Value::Matrix(a) => Ok(Value::Matrix(a.scale(-1.0))),
                },
                parser::UnaryOp::Factorial => match value {
                    Value::Scalar(n) => {
                        let factorial = scalars::factorial(n)?;
                        Ok(Value::Scalar(factorial))
                    }
                    Value::Matrix(_) => {
                        Err("No se puede calcular el factorial de una matriz".to_string())
                    }
                },
                parser::UnaryOp::Transpose => match value {
                    Value::Scalar(_) => {
                        Err("No se puede calcular la traspuesta de un número".to_string())
                    }
                    Value::Matrix(a) => Ok(Value::Matrix(a.transpose())),
                },
            }
        }
        AstNode::BinaryOp { left, op, right } => {
            let left = evaluate_expression(left)?;
            let right = evaluate_expression(right)?;
            match op {
                parser::BinaryOp::Add => match (left, right) {
                    (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a + b)),
                    (Value::Matrix(a), Value::Matrix(b)) => Ok(Value::Matrix(a.add(&b)?)),
                    _ => Err("No se puede sumar matrices con números".to_string()),
                },
                parser::BinaryOp::Subtract => match (left, right) {
                    (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a - b)),
                    (Value::Matrix(a), Value::Matrix(b)) => {
                        Ok(Value::Matrix(a.add(&b.scale(-1.0))?))
                    }
                    _ => Err("No se puede restar matrices con números".to_string()),
                },
                parser::BinaryOp::Multiply => return multiply(&left, &right),
                parser::BinaryOp::Divide => match right {
                    Value::Scalar(right) => {
                        if right == 0.0 {
                            return Err("No se puede dividir por cero".to_string());
                        }
                        multiply(&left, &Value::Scalar(1.0 / right))
                    }
                    Value::Matrix(a) => {
                        let right = a.inverse()?;
                        multiply(&left, &Value::Matrix(right))
                    }
                },
                parser::BinaryOp::RightDivide => match left {
                    Value::Scalar(left) => {
                        if left == 0.0 {
                            return Err("No se puede dividir por cero".to_string());
                        }
                        multiply(&Value::Scalar(1.0 / left), &right)
                    }
                    Value::Matrix(A) => {
                        let left = A.inverse()?;
                        multiply(&Value::Matrix(left), &right)
                    }
                },
                parser::BinaryOp::Power => {
                    if let Value::Scalar(right) = right {
                        match left {
                            Value::Scalar(left) => Ok(Value::Scalar(scalars::pow(left, right))),
                            Value::Matrix(a) => unimplemented!(),
                        }
                    } else {
                        Err("La potencia no puede ser una matriz".to_string())
                    }
                }
            }
        }
    }
}

fn multiply(left: &Value, right: &Value) -> ExprResult {
    match (left, right) {
        (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a * b)),
        (Value::Matrix(a), Value::Matrix(b)) => Ok(Value::Matrix(a.mul(&b)?)),
        (Value::Scalar(a), Value::Matrix(b)) => Ok(Value::Matrix(b.scale(*a))),
        (Value::Matrix(a), Value::Scalar(b)) => Ok(Value::Matrix(a.scale(*b))),
    }
}
