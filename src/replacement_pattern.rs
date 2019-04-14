use std::error::Error;
use std::fmt::{self, Display};
use std::num::NonZeroUsize;

#[derive(Debug, PartialEq)]
enum Token {
    Text(String),
    CaptureGroup(NonZeroUsize),
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
        unimplemented!()
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
            elements: vec![Token::CaptureGroup(NonZeroUsize::new(1).unwrap())],
        };

        let actual = ParserImpl::new().parse("$1").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_fails_with_0_group() {
        let expected = ParsingError::InvalidSyntax;

        let actual = ParserImpl::new().parse("foo-$0.bar").unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_with_multiple_groups() {
        let expected = Pattern {
            elements: vec![
                Token::Text(String::from("foo-")),
                Token::CaptureGroup(NonZeroUsize::new(1).unwrap()),
                Token::Text(String::from(".")),
                Token::CaptureGroup(NonZeroUsize::new(2).unwrap()),
            ],
        };

        let actual = ParserImpl::new().parse("foo-$1.$2").unwrap();

        assert_eq!(expected, actual);
    }
}
