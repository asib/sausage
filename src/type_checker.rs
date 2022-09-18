use std::collections::HashMap;

use crate::ast::{TypeExpr, AST};

#[derive(Debug, PartialEq)]
pub enum TypeError {
    UndefinedVariable(String),
    BadLet(String),
}

type Result<T> = std::result::Result<T, TypeError>;
struct TypeEnvironment(HashMap<String, TypeExpr>);

impl TypeEnvironment {
    pub fn new() -> TypeEnvironment {
        TypeEnvironment(HashMap::new())
    }
}

pub fn type_check(expression: &AST) -> Result<TypeExpr> {
    _type_check(&mut TypeEnvironment::new(), expression)
}

fn _type_check(environment: &mut TypeEnvironment, expression: &AST) -> Result<TypeExpr> {
    Ok(match expression {
        AST::Variable(name) => environment
            .0
            .get(name.to_owned())
            .ok_or(TypeError::UndefinedVariable(name.to_string()))?
            .clone(),
        AST::BooleanLiteral(_) => TypeExpr::Boolean,
        AST::StringLiteral(_) => TypeExpr::String,
        AST::NumberLiteral(_) => TypeExpr::Number,
        AST::Let {
            variable_declaration,
            variable_expression,
            expression,
        } => {
            let variable_type = _type_check(environment, &variable_expression)?;
            if variable_type != variable_declaration.typing {
                Err(TypeError::BadLet(format!("type of variable <{:?}: {:?}> in let expression did not match type of expression {:?}", variable_declaration.name, variable_declaration.typing, variable_type)))?
            } else {
                environment
                    .0
                    .insert(variable_declaration.name.to_owned(), variable_type);

                _type_check(environment, expression)?
            }
        }
        AST::Function { parameter, body } => {
            environment
                .0
                .insert(parameter.name.to_owned(), parameter.typing.clone());

            let body_type = _type_check(environment, body)?;

            TypeExpr::Function(Box::new(parameter.typing.clone()), Box::new(body_type))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::Result;
    use crate::parser::parse;
    use rstest::rstest;

    use super::{type_check, TypeError};

    #[rstest]
    #[case(r#"let x: number = true in 123"#, Err(TypeError::BadLet(r#"type of variable <"x": Number> in let expression did not match type of expression Boolean"#.to_owned())))]
    #[case(r#"let x: number = 123 in x"#, Ok(crate::ast::TypeExpr::Number))]
    #[case(r#"let x: number = 123 in "test""#, Ok(crate::ast::TypeExpr::String))]
    #[case(r#"let x: number = 123 in false"#, Ok(crate::ast::TypeExpr::Boolean))]
    #[case(
        r#"let x: number = 123 in let y: boolean = false in x"#,
        Ok(crate::ast::TypeExpr::Number)
    )]
    #[case(
        r#"let x: number = 123 in let y: boolean = false in y"#,
        Ok(crate::ast::TypeExpr::Boolean)
    )]
    #[case(r#"y: string"#, Err(TypeError::UndefinedVariable("y".to_owned())))]
    #[case("true", Ok(crate::ast::TypeExpr::Boolean))]
    #[case("8354584", Ok(crate::ast::TypeExpr::Number))]
    #[case(r#""true""#, Ok(crate::ast::TypeExpr::String))]
    #[case(
        r#"\x: string. \y: number. x"#,
        Ok(crate::ast::TypeExpr::Function(
            Box::new(crate::ast::TypeExpr::String),
            Box::new(crate::ast::TypeExpr::Function(
                Box::new(crate::ast::TypeExpr::Number),
                Box::new(crate::ast::TypeExpr::String)
            ))
        ))
    )]
    fn test_expr(#[case] input: &str, #[case] expected_result: Result<crate::ast::TypeExpr>) {
        let (_, program) = parse(input).unwrap();

        let result = type_check(&program);

        assert_eq!(result, expected_result)
    }
}
