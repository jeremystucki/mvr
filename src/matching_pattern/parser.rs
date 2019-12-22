use super::pattern::*;
use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::character::complete::char as nom_char;
use nom::combinator::{map, value};
use nom::multi::many1;
use nom::sequence::delimited;
use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::num::NonZeroUsize;

#[cfg(test)]
use mockiato::mockable;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ParsingError {
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

#[cfg_attr(test, mockable)]
pub(crate) trait Parser: Debug {
    fn parse(&self, input: &str) -> Result<Pattern, ParsingError>;
}

#[derive(Debug)]
pub(crate) struct ParserImpl {}

impl ParserImpl {
    pub(crate) fn new() -> Self {
        ParserImpl {}
    }
}

impl Parser for ParserImpl {
    fn parse(&self, input: &str) -> Result<Pattern, ParsingError> {
        let wildcard = value(Token::Wildcard, nom_char::<_, ()>('*'));

        let fixed_length = map(take_while1(|c| c == '?'), |input: &str| {
            Token::FixedLength(NonZeroUsize::new(input.len()).unwrap())
        });

        let text = map(
            take_while1(|c| !(c == '*' || c == '?' || c == '(' || c == ')')),
            |input: &str| Token::Text(String::from(input)),
        );

        let token = alt((wildcard, fixed_length, text));

        let group = delimited(
            nom_char('('),
            map(many1(&token), Element::Group),
            nom_char(')'),
        );

        let elements = many1(alt((group, map(&token, Element::Token))));

        let pattern = match elements(input).map_err(|_| ParsingError::InvalidSyntax)? {
            (remaining_text, _) if !remaining_text.is_empty() => {
                return Err(ParsingError::InvalidSyntax)
            }
            (_, elements) => Pattern { elements },
        };

        if contains_repeated_wildcards(&pattern) {
            return Err(ParsingError::InvalidSyntax);
        }

        Ok(pattern)
    }
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

    #[test]
    fn fails_with_repeated_wildcards_6() {
        let expected = ParsingError::InvalidSyntax;

        let actual = ParserImpl::new().parse("foo_(*??*).bar").unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_with_repeated_wildcards_7() {
        let expected = ParsingError::InvalidSyntax;

        let actual = ParserImpl::new().parse("foo_(*??)*.bar").unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_with_repeated_wildcards_8() {
        let expected = ParsingError::InvalidSyntax;

        let actual = ParserImpl::new().parse("foo_*??*.bar").unwrap_err();

        assert_eq!(expected, actual);
    }
}
