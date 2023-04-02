use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "parser/grammar.pest"]
pub struct ProgramParser;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum UnaryOp {
    Positive,
    Negate,
    Factorial,
    Transpose,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    RightDivide,
    Power,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Number(f64),
    Ident(String),
    Matrix(Vec<Vec<AstNode>>),
    Call {
        func: String,
        args: Vec<AstNode>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<AstNode>,
    },
    BinaryOp {
        op: BinaryOp,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub struct Statement {
    assign_to: Option<String>,
    pub expr: AstNode,
}

lazy_static::lazy_static! {
  static ref PRATT_PARSER: PrattParser<Rule> = {
      use pest::pratt_parser::{Assoc::*, Op};
      use Rule::*;

      // Precedence is defined lowest to highest
      PrattParser::new()
        .op(Op::infix(add, Left) | Op::infix(subtract, Left))
        .op(Op::infix(multiply, Left)
            | Op::infix(divide, Left)
            | Op::infix(right_divide, Left))
        .op(Op::infix(power, Right))
        .op(Op::postfix(factorial) | Op::postfix(transpose))
        .op(Op::prefix(positive) | Op::prefix(negative))
  };
}

fn parse_expr(pairs: Pairs<Rule>) -> AstNode {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::expr => parse_expr(primary.into_inner()),
            Rule::number => AstNode::Number(primary.as_str().parse::<f64>().unwrap()),
            Rule::ident => AstNode::Ident(primary.as_str().to_string()),
            Rule::matrix => {
                let mut pair = primary.into_inner();
                let mut elements: Vec<Vec<AstNode>> = vec![vec![]];
                while let Some(child) = pair.next() {
                    match child.as_rule() {
                        Rule::matrix_sep => {
                            if child.as_str() == ";" {
                                elements.push(Vec::<AstNode>::new());
                            }
                        }
                        Rule::expr => {
                            elements
                                .last_mut()
                                .unwrap()
                                .push(parse_expr(child.into_inner()));
                        }
                        rule => {
                            unreachable!("Unexpected atom when parsing a matrix, found {:?}", rule)
                        }
                    }
                }
                AstNode::Matrix(elements)
            }
            Rule::call => {
                let mut pair = primary.into_inner();
                let func = pair.next().unwrap();
                let mut args = Vec::<AstNode>::new();
                while let Some(arg) = pair.next() {
                    args.push(parse_expr(arg.into_inner()));
                }
                AstNode::Call {
                    func: func.as_str().to_string(),
                    args,
                }
            }
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|left, op, right| {
            let op = match op.as_rule() {
                Rule::add => BinaryOp::Add,
                Rule::subtract => BinaryOp::Subtract,
                Rule::multiply => BinaryOp::Multiply,
                Rule::divide => BinaryOp::Divide,
                Rule::right_divide => BinaryOp::RightDivide,
                Rule::power => BinaryOp::Power,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            AstNode::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            }
        })
        .map_prefix(|op, right| {
            let op = match op.as_rule() {
                Rule::positive => UnaryOp::Positive,
                Rule::negative => UnaryOp::Negate,
                _ => unreachable!(),
            };
            AstNode::UnaryOp {
                op,
                expr: Box::new(right),
            }
        })
        .map_postfix(|left, op| {
            let op = match op.as_rule() {
                Rule::factorial => UnaryOp::Factorial,
                Rule::transpose => UnaryOp::Transpose,
                _ => unreachable!(),
            };
            AstNode::UnaryOp {
                op,
                expr: Box::new(left),
            }
        })
        .parse(pairs)
}

pub fn parse(source: &str) -> Result<Vec<Statement>, pest::error::Error<Rule>> {
    let mut statements: Vec<Statement> = vec![];

    let pairs = ProgramParser::parse(Rule::program, source)?;
    for pair in pairs {
        let statement = match pair.as_rule() {
            Rule::assign => {
                let mut pairs = pair.into_inner();
                let ident = pairs.next().unwrap().as_str().to_string();
                let expr = parse_expr(pairs.next().unwrap().into_inner());
                Statement {
                    assign_to: Some(ident),
                    expr,
                }
            }
            Rule::expr => {
                let expr = parse_expr(pair.into_inner());
                Statement {
                    assign_to: None,
                    expr,
                }
            }
            Rule::EOI => break,
            _ => unreachable!(),
        };
        statements.push(statement);
    }

    Ok(statements)
}
