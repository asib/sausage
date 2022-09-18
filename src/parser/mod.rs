use super::ast::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    IResult,
};

mod combinator;

type ParseResult<'a, T> = IResult<&'a str, T>;

fn parse_type_expression(input: &str) -> ParseResult<TypeExpr> {
    alt((
        value(TypeExpr::Boolean, tag("boolean")),
        value(TypeExpr::String, tag("string")),
        value(TypeExpr::Number, tag("number")),
    ))(input)
}

fn parse_variable_declaration(input: &str) -> ParseResult<AST> {
    let (input, ident) = combinator::identifier(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, type_expr) = parse_type_expression(input)?;

    Ok((
        input,
        AST::VariableDeclaration {
            name: ident,
            typing: type_expr,
        },
    ))
}

fn parse_let_expr(input: &str) -> ParseResult<AST> {
    let (input, _) = tag("let ")(input)?;
    let (input, variable) = parse_variable_declaration(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, variable_expr) = parse(input)?;
    let (input, _) = tag(" in ")(input)?;
    let (input, expr) = parse(input)?;

    Ok((
        input,
        AST::Let {
            variable_declaration: Box::new(variable),
            variable_expression: Box::new(variable_expr),
            expression: Box::new(expr),
        },
    ))
}

pub fn parse(input: &str) -> IResult<&str, AST> {
    alt((
        combinator::boolean,
        combinator::string,
        combinator::number,
        parse_variable_declaration,
        parse_let_expr,
        map(combinator::identifier, AST::Variable),
    ))(input)
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::{parse, parse_let_expr, TypeExpr, AST};
    use rstest::rstest;

    #[rstest]
    #[case(
        r#"let x: string = "bla" in x"#,
        "x",
        TypeExpr::String,
        AST::StringLiteral("bla".to_owned()),
        AST::Variable("x"),
    )]
    fn test_let_expr(
        #[case] input: &str,
        #[case] variable_name: &str,
        #[case] variable_type: TypeExpr,
        #[case] variable_expression: AST,
        #[case] expression: AST,
    ) {
        let result = parse_let_expr(input);

        assert_eq!(
            result,
            Ok((
                "",
                AST::Let {
                    variable_declaration: Box::new(AST::VariableDeclaration {
                        name: variable_name,
                        typing: variable_type
                    }),
                    variable_expression: Box::new(variable_expression),
                    expression: Box::new(expression),
                }
            ))
        )
    }

    #[test]
    fn parse_other() {
        let result = parse("something");

        assert_matches!(result, Err(_))
    }
}
