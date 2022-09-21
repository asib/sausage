use super::ast::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    IResult,
};

mod combinator;

type ParseResult<'a, T> = IResult<&'a str, T>;

fn parse_function_type(input: &str) -> ParseResult<TypeExpr> {
    let (input, parameter_type) = parse_ground_type_expression(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, body_type) = parse_type_expression(input)?;

    Ok((
        input,
        TypeExpr::Function(Box::new(parameter_type), Box::new(body_type)),
    ))
}

fn parse_ground_type_expression(input: &str) -> ParseResult<TypeExpr> {
    alt((
        value(TypeExpr::Boolean, tag("boolean")),
        value(TypeExpr::String, tag("string")),
        value(TypeExpr::Number, tag("number")),
    ))(input)
}

fn parse_type_expression(input: &str) -> ParseResult<TypeExpr> {
    alt((parse_function_type, parse_ground_type_expression))(input)
}

fn parse_variable_declaration_with_type(input: &str) -> ParseResult<VariableDeclaration> {
    let (input, ident) = combinator::identifier(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, type_expr) = parse_type_expression(input)?;

    Ok((
        input,
        VariableDeclaration {
            name: ident,
            typing: Some(type_expr),
        },
    ))
}

fn parse_let_expr(input: &str) -> ParseResult<AST> {
    let (input, _) = tag("let ")(input)?;
    let (input, variable) = alt((
        parse_variable_declaration_with_type,
        map(combinator::identifier, |ident| VariableDeclaration {
            name: ident,
            typing: None,
        }),
    ))(input)?;
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

fn parse_function(input: &str) -> ParseResult<AST> {
    let (input, _) = tag("\\")(input)?;
    let (input, variable) = parse_variable_declaration_with_type(input)?;
    let (input, _) = tag(". ")(input)?;
    let (input, function_body) = parse(input)?;

    Ok((
        input,
        AST::Function {
            parameter: Box::new(variable),
            body: Box::new(function_body),
        },
    ))
}

pub fn parse(input: &str) -> ParseResult<AST> {
    alt((
        combinator::boolean,
        combinator::string,
        combinator::number,
        parse_let_expr,
        parse_function,
        map(combinator::identifier, AST::Variable),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::{
        parse_function, parse_let_expr, parse_type_expression, TypeExpr, VariableDeclaration, AST,
    };
    use rstest::rstest;

    #[rstest]
    #[case(
        r#"let x: string = "bla" in x"#,
        "x",
        TypeExpr::String,
        AST::StringLiteral("bla".to_owned()),
        AST::Variable("x"),
    )]
    #[case(
        r#"let x: string = "bla" in let y: number = 123 in y"#,
        "x",
        TypeExpr::String,
        AST::StringLiteral("bla".to_owned()),
        AST::Let { variable_declaration: Box::new(VariableDeclaration { name: "y", typing: Some(TypeExpr::Number) }), variable_expression: Box::new(AST::NumberLiteral("123".to_owned())), expression:Box::new(AST::Variable("y")) },
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
                    variable_declaration: Box::new(VariableDeclaration {
                        name: variable_name,
                        typing: Some(variable_type)
                    }),
                    variable_expression: Box::new(variable_expression),
                    expression: Box::new(expression),
                }
            ))
        )
    }

    #[rstest]
    #[case("let x = 5 in x", AST::Let { variable_declaration: Box::new(VariableDeclaration { name: "x", typing: None }), variable_expression: Box::new(AST::NumberLiteral("5".to_owned())), expression: Box::new(AST::Variable("x")) })]
    fn test_let_expr_without_type(#[case] input: &str, #[case] expected_result: AST) {
        let result = parse_let_expr(input);

        assert_eq!(result, Ok(("", expected_result)));
    }

    #[rstest]
    #[case(r#"\x: number. x"#, AST::Function { parameter: Box::new(VariableDeclaration { name: "x", typing: Some(TypeExpr::Number) }), body: Box::new(AST::Variable("x")) })]
    #[case(r#"\x: number. \y: string. x"#, AST::Function { parameter: Box::new(VariableDeclaration { name: "x", typing: Some(TypeExpr::Number) }), body: Box::new(AST::Function { parameter: Box::new(VariableDeclaration { name: "y", typing: Some(TypeExpr::String) }), body: Box::new(AST::Variable("x")) }) })]
    fn test_function<'a>(#[case] input: &str, #[case] expected_result: AST<'a>) {
        let result = parse_function(input);

        assert_eq!(result, Ok(("", expected_result)));
    }

    #[rstest]
    #[case("boolean", TypeExpr::Boolean)]
    #[case("string", TypeExpr::String)]
    #[case("number", TypeExpr::Number)]
    #[case(
        "string -> number",
        TypeExpr::Function(Box::new(TypeExpr::String), Box::new(TypeExpr::Number))
    )]
    #[case(
        "string -> number -> boolean",
        TypeExpr::Function(
            Box::new(TypeExpr::String),
            Box::new(TypeExpr::Function(Box::new(TypeExpr::Number), Box::new(TypeExpr::Boolean)))
        )
    )]
    fn test_type_expression(#[case] input: &str, #[case] expected_result: TypeExpr) {
        let result = parse_type_expression(input);

        assert_eq!(result, Ok(("", expected_result)));
    }
}
