use nom::types::CompleteStr;
use std::error::Error;
use std::fmt::{self, Display};
use std::num::NonZeroUsize;

#[derive(Debug, PartialEq)]
pub enum Token {
    Text(String),
    FixedLength(NonZeroUsize),
    Wildcard,
}

#[derive(Debug, PartialEq)]
pub enum Element {
    Token(Token),
    Group(Vec<Token>),
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    pub elements: Vec<Element>,
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

#[derive(Debug, Default)]
pub struct ParserImpl {}

impl ParserImpl {
    fn new() -> Self {
        ParserImpl::default()
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
                many1!(token), Element::Group),
            char!(')')
        ));

        named!(elements<CompleteStr, Vec<Element>>,
        many1!(
            alt!(
                group | map!(token, Element::Token)
            )
        ));

        let pattern = match elements(CompleteStr(input)) {
            Err(_) => Err(ParsingError::InvalidSyntax)?,
            Ok((remaining_text, elements)) => {
                if remaining_text.len() > 0 {
                    Err(ParsingError::InvalidSyntax)?
                } else {
                    Pattern { elements }
                }
            }
        };

        if contains_repeated_wildcards(&pattern) {
            Err(ParsingError::InvalidSyntax)?
        }

        Ok(pattern)
    }
}

fn contains_repeated_wildcards(pattern: &Pattern) -> bool {
    let mut tokens: Vec<_> = pattern
        .elements
        .iter()
        .flat_map(|element| match element {
            Element::Token(token) => vec![token],
            Element::Group(tokens) => tokens.iter().collect(), // TODO
        })
        .collect();

    let number_of_tokens = tokens.len();

    tokens.dedup();

    number_of_tokens != tokens.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wildcard_pattern() {
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
    fn wildcard_pattern_inside_capture_group() {
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
    fn fixed_length_pattern() {
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
    fn text_only() {
        let expected = Pattern {
            elements: vec![Element::Token(Token::Text(String::from("foo.bar")))],
        };

        let actual = ParserImpl::new().parse("foo.bar").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn wildcard_and_fixed_length_token_in_capture_group() {
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
    fn fails_with_mismatched_grouping() {
        let expected = ParsingError::InvalidSyntax;

        let actual = ParserImpl::new().parse("foo_(??.*").unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_with_repeated_wildcards_1() {
        let expected = ParsingError::InvalidSyntax;

        let actual = ParserImpl::new().parse("foo_*(*).bar").unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_with_repeated_wildcards_2() {
        let expected = ParsingError::InvalidSyntax;

        let actual = ParserImpl::new().parse("foo_**.bar").unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_with_repeated_wildcards_3() {
        let expected = ParsingError::InvalidSyntax;

        let actual = ParserImpl::new().parse("foo_(*)*.bar").unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_with_repeated_wildcards_4() {
        let expected = ParsingError::InvalidSyntax;

        let actual = ParserImpl::new().parse("foo_(*)(*).bar").unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_with_repeated_wildcards_5() {
        let expected = ParsingError::InvalidSyntax;

        let actual = ParserImpl::new().parse("foo_(**).bar").unwrap_err();

        assert_eq!(expected, actual);
    }
}
