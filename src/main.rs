mod parser;

use parser::parse;
use std::io::{stdin, stdout, Write};

use crate::parser::AstNode;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

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
        AstNode::BinaryOp {left, op, right} => {
            
            let iz = print_expr(left);
            let de = print_expr(right);
            match op {
                parser::BinaryOp::Add => return iz + de,
                parser::BinaryOp::Subtract => return iz - de,
                parser::BinaryOp::Multiply => return iz * de,
                parser::BinaryOp::Divide => return iz / de,
                parser::BinaryOp::Power => return iz.powf(de),
                parser::BinaryOp::RightDivide => return iz % de, // right divide es el modulo?
            }
        }
        _ => println!("otra cosa"),
    }
    println!("\n");
    3.14
}
