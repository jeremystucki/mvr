use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::character::complete::{char as nom_char, digit1};
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::preceded;
use std::error::Error;
use std::fmt::{self, Debug, Display};

#[cfg(test)]
use mockiato::mockable;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Element {
    Text(String),
    CaptureGroup(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Pattern {
    pub(crate) elements: Vec<Element>,
}

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
        Self {}
    }
}

impl Parser for ParserImpl {
    fn parse(&self, input: &str) -> Result<Pattern, ParsingError> {
        let text = map(take_while1::<_, _, ()>(|c| c != '$'), |input| {
            Element::Text(String::from(input))
        });

        let capture_group = preceded(
            nom_char('$'),
            map(digit1, |input: &str| {
                Element::CaptureGroup(input.parse().unwrap())
            }),
        );

        let elements = many1(alt((capture_group, text)));

        let pattern = match elements(input).map_err(|_| ParsingError::InvalidSyntax)? {
            (remaining_text, _) if !remaining_text.is_empty() => {
                return Err(ParsingError::InvalidSyntax)
            }
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
            elements: vec![Element::Text(String::from("foo.bar"))],
        };

        let actual = ParserImpl::new().parse("foo.bar").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_with_group_only() {
        let expected = Pattern {
            elements: vec![Element::CaptureGroup(0)],
        };

        let actual = ParserImpl::new().parse("$0").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_with_multiple_groups() {
        let expected = Pattern {
            elements: vec![
                Element::Text(String::from("foo-")),
                Element::CaptureGroup(1),
                Element::Text(String::from(".")),
                Element::CaptureGroup(2),
            ],
        };

        let actual = ParserImpl::new().parse("foo-$1.$2").unwrap();

        assert_eq!(expected, actual);
    }
}
