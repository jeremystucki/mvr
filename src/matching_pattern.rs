use std::error::Error;
use std::fmt::{self, Display};
use std::num::NonZeroUsize;

#[derive(Debug)]
enum Token {
    Text(String),
    FixedLength(NonZeroUsize),
    Wildcard,
}

#[derive(Debug)]
enum Element {
    Token(Token),
    Group(Vec<Token>),
}

#[derive(Debug)]
struct MatchingPattern {
    elements: Vec<Element>,
}

#[derive(Debug)]
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

trait MatchingPatternParser {
    fn parse(&self, input: &str) -> Result<MatchingPattern, ParsingError>;
}

struct ReplacementPatternParserImpl {}

impl MatchingPatternParser for ReplacementPatternParserImpl {
    fn parse(&self, input: &str) -> Result<MatchingPattern, ParsingError> {
        unimplemented!()
    }
}
