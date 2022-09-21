#![feature(assert_matches)]
use crate::{parser::parse, type_checker::type_check};

mod ast;
mod parser;
mod type_checker;

fn main() {
    if let Ok((_, program)) =
        parse(r#"let x: boolean -> number = \q: boolean. 123 in \y: string. x"#)
    {
        if let Ok(typing) = type_check(&program) {
            println!("{:?}", typing);
        }
        println!("{:?}", program);
    }
}
