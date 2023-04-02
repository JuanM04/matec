mod matrix;
mod parser;

use parser::parse;
// use core::num::dec2flt::number::Number;
use std::io::{stdin, stdout, Write};

use crate::parser::AstNode;

fn print_type_of<T>(_: &T) {
    println!("Type of: \n{:#?}\n", std::any::type_name::<T>())
}

fn main() {
    loop {
        print!("> ");
        let mut input = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            println!("\n[!] Saliendo... \n\n");
            break
        }

        let result = parse(&input);
        if let Ok(ast) = result {
            println!("Parsed! {:#?}", ast);
            //print_type_of(&ast);
            //print_type_of(&ast[0]);
            //print_type_of(&ast[0].expr);
            for n in &ast {
                println!("Result: {}",evaluate_expr_without_matrix(&n.expr));
            }
        } else {
            println!("Error: {:#?}", result);
        }
    }
}


fn evaluate_expr_without_matrix(node: &AstNode) -> f64 {
    match node {
        AstNode::Number(n) => {
            return *n;
        },
        AstNode::Matrix(A) => evaluate_matrix(A),
        AstNode::UnaryOp {op, expr} => {
            match op {
                parser::UnaryOp::Positive => return evaluate_expr_without_matrix(expr),
                parser::UnaryOp::Negate => return evaluate_expr_without_matrix(expr) * -1.0,
                parser::UnaryOp::Factorial => return calculate_factorial(evaluate_expr_without_matrix(expr)) as f64,
                parser::UnaryOp::Transpose => {
                    print_type_of(expr);
                    calculate_transpose(expr);
                    return 2.71;
                }, // Todo chequar que esto sea una matriz y 
            }
        }
        AstNode::BinaryOp {left, op, right} => {
            
            let iz = evaluate_expr_without_matrix(left);
            let de = evaluate_expr_without_matrix(right);
            match op {
                parser::BinaryOp::Add => return iz + de,
                parser::BinaryOp::Subtract => return iz - de,
                parser::BinaryOp::Multiply => return iz * de,
                parser::BinaryOp::Divide => {
                    if de != 0.0 {    
                        return iz / de;
                    } else {
                        panic!("[!] No se puede dividir por cero!!!");
                    }
                },
                parser::BinaryOp::Power => return iz.powf(de),
                parser::BinaryOp::RightDivide => {
                    if iz != 0.0 {
                        return de / iz;
                    } else {
                        panic!("[!] No se puede dividir por cero!!!");
                    }
                },
            }
        }
        _ => println!("otra cosa"),
    }
    println!("\n");
    3.14
}


fn calculate_factorial(num: f64) -> u64 {

    let mut factorial: u64 = 1;
    let num_int: u64 = num.trunc() as u64;

    if num.fract() != 0.0 {
        // TODO decidir que hacer cuando uno no es muy bueno en las matematicas
        eprintln!("[!] El número {} tiene decimales, no se puede calcular Factorial.\n    Se calculara el factorial del entero redondeado abajo: {}.", num, num_int);
    } 
    
    for n in 2..=num_int {
        factorial *= n;
    }
    
    factorial 
}

fn calculate_transpose(node: &AstNode){
    println!("Ingreso CalculateTranspose");
    print_type_of(node);
    match node {
        AstNode::Matrix(A) => print_type_of(A),
        _ => println!("No matriz"),
    }
} 

/* fn calculate_transpose_matrix(matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let rows = matrix.len();
    let columns = matrix[0].len();
    
    let mut transpose = vec![vec![0.0; rows]; columns];
    
    for i in 0..rows {
        for j in 0..columns {
            transpose[j][i] = matrix[i][j];
        }
    }
    
    transpose
} */


fn evaluate_matrix (matrix: &Vec<Vec<AstNode>>) {
    let rows:usize = matrix.len();
    let columns:usize = matrix[0].len();
    
    println!("Tipo de Matriz");
    print_type_of(&matrix);
    println!("Tipo de Matriz[0]");
    print_type_of(&matrix[0]);
    println!("Tipo de Matriz[0][0]");
    print_type_of(&matrix[0][0]);

    for i in 0..rows {
        for j in 0..columns {
            println!("Numero que deberia guardarse en la posicion [{}][{}]: {}",i,j,evaluate_expr_without_matrix(&matrix[i][j]));
        }
    }

} 
