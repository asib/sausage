#![feature(assert_matches)]
use std::collections::HashMap;

use crate::{parser::parse, type_checker::type_check};

mod ast;
mod parser;
mod type_checker;

fn main() {
    if let Ok((_, program)) = parse("let x: string = TRUE in x") {
        if let Err(err) = type_check(&program) {
            panic!("{:?}", err);
        }
        println!("{:?}", program);
    }
}
