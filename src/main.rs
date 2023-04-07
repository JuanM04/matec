mod functions;
mod matrix;
mod parser;
mod utils;
mod value;

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
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
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

/// Evalúa una expresión y devuelve el resultado.
/// Esta es una función recursiva que evalúa cada nodo del AST.
/// Puede devolver un error si la expresión no es válida.
fn evaluate_expression(expr: &AstNode, variables: &Variables) -> Result<Value, String> {
    match expr {
        // Si el nodo es una variable, se busca en el hashmap de variables.
        AstNode::Ident(s) => {
            if let Some(v) = variables.get(s) {
                Ok(v.clone())
            } else {
                Err(format!("La variable \"{}\" no está definida", s))
            }
        }
        // Si el nodo es un número, se devuelve el valor.
        AstNode::Scalar(n) => Ok(Value::Scalar(*n)),
        // Si el nodo es una matriz, se pasa a Matrix.
        AstNode::Matrix(vec) => {
            // Se recibe un vector de vectores de nodos. Vec<Vec<AstNode>>
            // El primer vector representa las filas de la matriz.
            // El segundo vector representa las columnas de la matriz.
            // Por ejemplo, la matriz [[1, 2], [3, 4]] se representa como:
            // vec![vec![AstNode::Scalar(1), AstNode::Scalar(2)], vec![AstNode::Scalar(3), AstNode::Scalar(4)]]

            // Hay que verificar que la matriz esté bien declarada.
            // Primero, se verifica el caso de una matriz vacía.
            let rows = vec.len();
            if rows == 0 {
                return Ok(Value::Matrix(Matrix::new(0, 0)));
            }

            // Luego, se toma el número de columnas de la primera fila.
            // Si alguna fila tiene un número distinto de columnas, se devuelve un error.
            let cols = vec[0].len();
            let mut matrix = Matrix::new(rows, cols);

            // Se iteran las filas de la matriz.
            for (i, row) in vec.iter().enumerate() {
                // Si una fila tiene una cantidad distinta de columnas a la primera fila,
                // se devuelve un error.
                if row.len() != cols {
                    return Err(
                        "La matriz está mal declarada: el número de columnas no es consistente"
                            .to_string(),
                    );
                }

                // Se itera cada columna de la fila.
                for (j, col) in row.iter().enumerate() {
                    // Dada la recursividad de la función, se evalúa cada elemento de la matriz.
                    // Por ejemplo, se puede tener una matriz [1, 2; 5*4, 3]
                    // donde 5*4 es una expresión que se evalúa recursivamente.

                    // Se evalúa la expresión y se guarda en la matriz.
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
            // Se devuelve la matriz.
            Ok(Value::Matrix(matrix))
        }
        // Se encontró un operador unario. (Como -5, o 5!)
        // Todas funciones unarias se encuentran en functions/mod.rs
        AstNode::UnaryOp { op, expr } => {
            let value = evaluate_expression(expr, variables)?;
            match op {
                parser::UnaryOp::Positive => Ok(value),
                parser::UnaryOp::Negate => functions::negate(&value),
                parser::UnaryOp::Factorial => functions::factorial(&value),
                parser::UnaryOp::Transpose => functions::transpose(&value),
            }
        }
        // Se encontró un operador binbario. (Como 4-5, o 3^2)
        // Todas las funciones binarias se encuentran en functions/mod.rs
        AstNode::BinaryOp { left, op, right } => {
            let left = evaluate_expression(left, variables)?;
            let right = evaluate_expression(right, variables)?;
            match op {
                parser::BinaryOp::Add => functions::add(&left, &right),
                parser::BinaryOp::Subtract => functions::subtract(&left, &right),
                parser::BinaryOp::Multiply => functions::multiply(&left, &right),
                parser::BinaryOp::Divide => functions::divide(&left, &right),
                parser::BinaryOp::RightDivide => functions::right_divide(&left, &right),
                parser::BinaryOp::Power => functions::pow(&left, &right),
            }
        }

        // Se econtró una función. (Como sin(5), o det(A))
        // Todas las funciones se encuentran en functions/mod.rs
        AstNode::Call { func, args } => {
            // Primero, se evalúa cada argumento de la función.
            let mut evaluated_args: Vec<Value> = Vec::new();
            for arg in args {
                evaluated_args.push(evaluate_expression(arg, variables)?);
            }

            let name = func.as_str();

            // Se llama a la función correspondiente.
            match name {
                "abs" => {
                    if evaluated_args.len() != 1 {
                        return Err("La función abs() recibe un argumento".to_string());
                    }
                    functions::abs(&evaluated_args[0])
                }
                "sqrt" => {
                    if evaluated_args.len() != 1 {
                        return Err("La función sqrt() recibe un argumento".to_string());
                    }
                    functions::sqrt(&evaluated_args[0])
                }
                "pow" => {
                    if evaluated_args.len() != 2 {
                        return Err("La función pow() recibe dos argumentos".to_string());
                    }
                    functions::pow(&evaluated_args[0], &evaluated_args[1])
                }
                "inv" => {
                    if evaluated_args.len() != 1 {
                        return Err("La función inv() recibe un argumento".to_string());
                    }
                    functions::inverse(&evaluated_args[0])
                }
                "factorial" => {
                    if evaluated_args.len() != 1 {
                        return Err("La función factorial() recibe un argumento".to_string());
                    }
                    functions::factorial(&evaluated_args[0])
                }
                "sin" => {
                    if evaluated_args.len() != 1 {
                        return Err("La función sin() recibe un argumento".to_string());
                    }
                    functions::sin(&evaluated_args[0])
                }
                "cos" => {
                    if evaluated_args.len() != 1 {
                        return Err("La función cos() recibe un argumento".to_string());
                    }
                    functions::cos(&evaluated_args[0])
                }
                "tan" => {
                    if evaluated_args.len() != 1 {
                        return Err("La función tan() recibe un argumento".to_string());
                    }
                    functions::tan(&evaluated_args[0])
                }
                "log" => {
                    if evaluated_args.len() != 1 {
                        return Err("La función log() recibe un argumento".to_string());
                    }
                    functions::log(&evaluated_args[0])
                }
                "transpose" => {
                    if evaluated_args.len() != 1 {
                        return Err("La función transpose() recibe un argumento".to_string());
                    }
                    functions::transpose(&evaluated_args[0])
                }
                "det" => {
                    if evaluated_args.len() != 1 {
                        return Err("La función det() recibe un argumento".to_string());
                    }
                    functions::det(&evaluated_args[0])
                }
                "linsolve" => {
                    if evaluated_args.len() != 1 {
                        return Err("La función linsolve() recibe dos argumentos".to_string());
                    }
                    functions::linsolve(&evaluated_args[0], &evaluated_args[1])
                }
                _ => Err(format!("La función {} no está definida", name)),
            }
        }
    }
}
