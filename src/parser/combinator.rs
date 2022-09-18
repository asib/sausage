use super::{ParseResult, TBoolean, AST};
use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_not, tag, tag_no_case},
    character::complete::{alpha1, alphanumeric1, char, digit1},
    combinator::{map, recognize, value},
    multi::many0,
    sequence::{delimited, pair},
};

fn boolean_true(input: &str) -> ParseResult<TBoolean> {
    value(TBoolean::True, tag_no_case("true"))(input)
}

fn boolean_false(input: &str) -> ParseResult<TBoolean> {
    value(TBoolean::False, tag_no_case("false"))(input)
}

pub fn boolean(input: &str) -> ParseResult<AST> {
    map(alt((boolean_true, boolean_false)), AST::BooleanLiteral)(input)
}

pub fn identifier(input: &str) -> ParseResult<&str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

pub fn number(input: &str) -> ParseResult<AST> {
    map(digit1, |val: &str| AST::NumberLiteral(val.to_owned()))(input)
}

pub fn string(input: &str) -> ParseResult<AST> {
    map(
        delimited(
            char('"'),
            escaped_transform(
                is_not("\"\\"),
                '\\',
                alt((value("\\", char('\\')), value("\"", char('"')))),
            ),
            char('"'),
        ),
        AST::StringLiteral,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::{boolean, identifier, number, string, TBoolean, AST};
    use rstest::rstest;
    use std::assert_matches::assert_matches;

    #[rstest]
    #[case("true", TBoolean::True)]
    #[case("false", TBoolean::False)]
    #[case("TRUE", TBoolean::True)]
    #[case("FALSE", TBoolean::False)]
    #[case("TrUe", TBoolean::True)]
    #[case("FaLsE", TBoolean::False)]
    fn test_boolean(#[case] program: &str, #[case] expected_result: TBoolean) {
        let result = boolean(program);

        assert_eq!(result, Ok(("", AST::BooleanLiteral(expected_result))));
    }

    #[rstest]
    #[case(r#""test""#, "test")]
    #[case(r#""hello, \"world\"""#, r#"hello, "world""#)]
    fn test_string_literal(#[case] input: &str, #[case] expected_result: &str) {
        let result = string(input);

        assert_eq!(
            result,
            Ok(("", AST::StringLiteral(expected_result.to_owned())))
        );
    }

    #[rstest]
    #[case("15", "15")]
    #[case(
        "128458457843584359843594359084398534985",
        "128458457843584359843594359084398534985"
    )]
    fn test_number_literal(#[case] input: &str, #[case] expected_result: &str) {
        let result = number(input);

        assert_eq!(
            result,
            Ok(("", AST::NumberLiteral(expected_result.to_owned())))
        );
    }

    #[rstest]
    #[case("testing123", "testing123")]
    #[case("_ident", "_ident")]
    #[case("my_ident", "my_ident")]
    fn test_identifier(#[case] input: &str, #[case] expected_result: &str) {
        let result = identifier(input);

        assert_eq!(result, Ok(("", expected_result)));
    }

    #[rstest]
    #[case("123testing")]
    fn test_not_identifier(#[case] input: &str) {
        let result = identifier(input);

        assert_matches!(result, Err(_))
    }
}
