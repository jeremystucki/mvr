use nom::types::CompleteStr;
use std::error::Error;
use std::fmt::{self, Display};
use std::num::NonZeroUsize;

#[derive(Debug, PartialEq)]
enum Token {
    Text(String),
    FixedLength(NonZeroUsize),
    Wildcard,
}

#[derive(Debug, PartialEq)]
enum Element {
    Token(Token),
    Group(Vec<Token>),
}

#[derive(Debug, PartialEq)]
struct Pattern {
    elements: Vec<Element>,
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
        named!(wildcard<CompleteStr, Token>,
        value!(
            Token::Wildcard,
            char!('*')
        ));

        named!(fixed_length<CompleteStr, Token>,
        map!(
            take_while1!(
                |character| character == '?'),
            |string| Token::FixedLength(NonZeroUsize::new(string.len()).unwrap())
        ));

        named!(text<CompleteStr, Token>,
        map!(
            take_while1!(
                |c| !(c == '*' || c == '?' || c == '(' || c == ')')),
            |complete_string| Token::Text(complete_string.to_string())
        ));

        named!(token<CompleteStr, Token>,
        alt!(wildcard | fixed_length | text));

        named!(group<CompleteStr, Element>,
        delimited!(
            char!('('),
            map!(
                many1!(token), |tokens| Element::Group(tokens)),
            char!(')')
        ));

        named!(elements<CompleteStr, Vec<Element>>,
        many1!(
            alt!(
                group | map!(token, |token| Element::Token(token))
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
    fn parse_with_wildcard_pattern() {
        let expected = Pattern {
            elements: vec![
                Element::Token(Token::Text(String::from("foo"))),
                Element::Token(Token::Wildcard),
                Element::Token(Token::Text(String::from(".bar"))),
            ],
        };

        let actual = ParserImpl::new().parse("foo*.bar").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_with_wildcard_pattern_inside_capture_group() {
        let expected = Pattern {
            elements: vec![
                Element::Token(Token::Text(String::from("foo"))),
                Element::Group(vec![Token::Wildcard]),
                Element::Token(Token::Text(String::from(".bar"))),
            ],
        };

        let actual = ParserImpl::new().parse("foo(*).bar").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_with_fixed_length_pattern() {
        let expected = Pattern {
            elements: vec![
                Element::Token(Token::Text(String::from("foo_"))),
                Element::Token(Token::FixedLength(NonZeroUsize::new(2).unwrap())),
                Element::Token(Token::Text(String::from(".bar"))),
            ],
        };

        let actual = ParserImpl::new().parse("foo_??.bar").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_with_text_only() {
        let expected = Pattern {
            elements: vec![Element::Token(Token::Text(String::from("foo.bar")))],
        };

        let actual = ParserImpl::new().parse("foo.bar").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_with_wildcard_and_fixed_length_token_in_capture_group() {
        let expected = Pattern {
            elements: vec![
                Element::Token(Token::Text(String::from("foo_"))),
                Element::Group(vec![
                    Token::FixedLength(NonZeroUsize::new(2).unwrap()),
                    Token::Text(String::from(".")),
                    Token::Wildcard,
                ]),
            ],
        };

        let actual = ParserImpl::new().parse("foo_(??.*)").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_fails_with_mismatched_grouping() {
        let expected = ParsingError::InvalidSyntax;

        let actual = ParserImpl::new().parse("foo_(??.*").unwrap_err();

        assert_eq!(expected, actual);
    }
}
