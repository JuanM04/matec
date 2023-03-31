mod parser;

use parser::parse;
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!("> ");
        let mut input = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        let result = parse(&input);
        if let Ok(ast) = result {
            println!("Parsed! {:#?}", ast);
        } else {
            println!("Error: {:#?}", result);
        }
    }
}
