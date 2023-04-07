mod functions;
mod matrix;
mod parser;
mod value;

use functions::scalars;
use matrix::Matrix;
use parser::{parse, AstNode};
use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};
use value::Value;

type Variables = HashMap<String, Value>;

fn main() {
    // En este hashmap se guardan las variables que se van creando.
    let mut variables: Variables = HashMap::new();

    // Agregamos las variables pi y e.
    variables.insert("pi".to_string(), Value::Scalar(std::f64::consts::PI));
    variables.insert("e".to_string(), Value::Scalar(std::f64::consts::E));

    // Impresión del mensaje de bienvenida.
    println!("#===========================#");
    println!("# Clon sin nombre de Matlab #");
    println!("#===========================#");
    println!("");
    println!("Por Majoros, Lorenzo; y Seery, Juan Martín");
    println!("Para Matemática C - 2023");
    println!("");
    println!("Para salir, escriba \"exit\"");
    println!("");
    println!("");

    loop {
        // Se lee la entrada del usuario.
        print!("> ");
        let mut input = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Casos especiales de comandos.
        if input == "exit" {
            break;
        } else if input == "clc" {
            println!("\x1B[2J\x1B[1;1H");
            continue;
        }

        // Se parsea la entrada en texto a un AST (ver parser/mod.rs)
        match parse(&input) {
            // Si no hay errores de sintáxis, se evalúa cada expresión.
            Ok(ast) => {
                let len = ast.len();
                for i in 0..len {
                    // Si la expresión tiene asignación (x = ...), se toma el nombre de la variable.
                    // De lo contrario, se asigna a la variable "ans".
                    let assign_to = &ast[i].assign_to.clone().unwrap_or("ans".to_string());
                    let expr = &ast[i].expr;
                    // Se evalúa la expresión.
                    match evaluate_expression(expr, &variables) {
                        Ok(ans) => {
                            if i + 1 == len {
                                // Si es la última expresión, se imprime el resultado.
                                println!("{} = {}", assign_to, ans);
                            }
                            // Se guarda el resultado en el hashmap de variables.
                            variables.insert(assign_to.to_string(), ans);
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                            break;
                        }
                    }
                }
            }
            // Si hay un error de sintáxis, se imprime el error.
            Err(e) => println!("Error de sintáxis: {:#?}", e),
        };
    }
}

type ExprResult = Result<Value, String>;

fn evaluate_expression(expr: &AstNode, variables: &Variables) -> ExprResult {
    match expr {
        AstNode::Ident(s) => {
            if let Some(v) = variables.get(s) {
                Ok(v.clone())
            } else {
                Err(format!("La variable \"{}\" no está definida", s))
            }
        }
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

                    match evaluate_expression(col, variables) {
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
            let value = evaluate_expression(expr, variables)?;
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
            let left = evaluate_expression(left, variables)?;
            let right = evaluate_expression(right, variables)?;
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
