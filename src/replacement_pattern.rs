use nom::digit;
use nom::types::CompleteStr;
use std::error::Error;
use std::fmt::{self, Display};
use std::num::NonZeroUsize;

#[derive(Debug, PartialEq)]
pub enum Token {
    Text(String),
    CaptureGroup(NonZeroUsize),
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    elements: Vec<Token>,
}

#[derive(Debug, PartialEq)]
pub enum ParsingError {
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

pub trait Parser {
    fn parse(&self, input: &str) -> Result<Pattern, ParsingError>;
}

pub struct ParserImpl {}

impl ParserImpl {
    fn new() -> Self {
        Self {}
    }
}

impl Parser for ParserImpl {
    fn parse(&self, input: &str) -> Result<Pattern, ParsingError> {
        named!(text<CompleteStr, Token>,
        map!(
            take_while1!(
                |character| character != '$'),
            |complete_string| Token::Text(complete_string.to_string())
        ));

        named!(capture_group<CompleteStr, Token>,
        preceded!(
            char!('$'),
            map!(
                digit, // TODO: Look at the Endianness
                |index| Token::CaptureGroup(
                    NonZeroUsize::new(index.parse().unwrap()).expect("Capture group indices start at 1"))) // TODO: Find a way to do this without panicking
        ));

        named!(elements<CompleteStr, Vec<Token>>,
        many1!(
            alt!(
                capture_group | text
            )
        ));

        match elements(CompleteStr(input)) {
            Err(_) => Err(ParsingError::InvalidSyntax),
            Ok((remaining_text, elements)) => {
                if remaining_text.len() > 0 {
                    Err(ParsingError::InvalidSyntax)
                } else {
                    Ok(Pattern { elements })
                }
            }
        }
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
    #[should_panic]
    fn parse_fails_with_0_group() {
        let _ = ParserImpl::new().parse("foo-$0.bar").unwrap_err();
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
