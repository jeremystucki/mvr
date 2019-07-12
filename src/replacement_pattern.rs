use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::character::complete::{char as nom_char, digit1};
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::preceded;
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug, PartialEq)]
enum Token {
    Text(String),
    CaptureGroup(usize),
}

#[derive(Debug, PartialEq)]
struct Pattern {
    elements: Vec<Token>,
}

#[derive(Debug, PartialEq)]
enum ParsingError {
    InvalidSyntax,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            ParsingError::InvalidSyntax => "The given pattern is not valid",
        };

        write!(f, "{}", message)
    }
}

impl Error for ParsingError {}

trait Parser {
    fn parse(&self, input: &str) -> Result<Pattern, ParsingError>;
}

struct ParserImpl {}

impl ParserImpl {
    fn new() -> Self {
        Self {}
    }
}

impl Parser for ParserImpl {
    fn parse(&self, input: &str) -> Result<Pattern, ParsingError> {
        let text = map(take_while1::<_, _, ()>(|c| c != '$'), |input: &str| {
            Token::Text(String::from(input))
        });

        let capture_group = preceded(
            nom_char('$'),
            map(digit1, |input: &str| {
                Token::CaptureGroup(input.parse().unwrap())
            }),
        );

        let elements = many1(alt((capture_group, text)));

        let pattern = match elements(input).map_err(|_| ParsingError::InvalidSyntax)? {
            (remaining_text, _) if !remaining_text.is_empty() => Err(ParsingError::InvalidSyntax)?,
            (_, elements) => Pattern { elements },
        };

        Ok(pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_with_text_only() {
        let expected = Pattern {
            elements: vec![Token::Text(String::from("foo.bar"))],
        };

        let actual = ParserImpl::new().parse("foo.bar").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_with_group_only() {
        let expected = Pattern {
            elements: vec![Token::CaptureGroup(0)],
        };

        let actual = ParserImpl::new().parse("$0").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_with_multiple_groups() {
        let expected = Pattern {
            elements: vec![
                Token::Text(String::from("foo-")),
                Token::CaptureGroup(1),
                Token::Text(String::from(".")),
                Token::CaptureGroup(2),
            ],
        };

        let actual = ParserImpl::new().parse("foo-$1.$2").unwrap();

        assert_eq!(expected, actual);
    }
}
