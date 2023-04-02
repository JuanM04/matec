mod parser;

use parser::parse;
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
                println!("Result: {}",print_expr(&n.expr));
            }
        } else {
            println!("Error: {:#?}", result);
        }
    }
}


fn print_expr(node: &AstNode) -> f64 {
    match node {
        AstNode::Number(n) => {
            return *n;
        },
        AstNode::UnaryOp {op, expr} => {
            match op {
                parser::UnaryOp::Positive => return print_expr(expr),
                parser::UnaryOp::Negate => return print_expr(expr) * -1.0,
                parser::UnaryOp::Factorial => return calculate_factorial(print_expr(expr)) as f64,
                parser::UnaryOp::Transpose => return print_expr(expr), // Todo chequar que esto sea una matriz y 
            }
        }
        AstNode::BinaryOp {left, op, right} => {
            
            let iz = print_expr(left);
            let de = print_expr(right);
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
    
    return factorial 
    
}