#![feature(assert_matches)]
use crate::parser::parse;

mod ast;
mod parser;

fn main() {
    if let Ok((_, program)) = parse("let x: boolean = TRUE in x") {
        println!("{:?}", program);
    }
}
