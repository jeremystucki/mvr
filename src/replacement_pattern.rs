use std::error::Error;
use std::fmt::{self, Display};
use std::num::NonZeroUsize;

#[derive(Debug)]
enum Token {
    Text(String),
    CaptureGroup(NonZeroUsize),
}

#[derive(Debug)]
pub struct ReplacementPattern {
    elements: Vec<Token>,
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

trait ReplacementPatternParser {
    fn parse(&self, input: &str) -> Result<ReplacementPattern, ParsingError>;
}

struct ReplacementPatternParserImpl {}

impl ReplacementPatternParser for ReplacementPatternParserImpl {
    fn parse(&self, _input: &str) -> Result<ReplacementPattern, ParsingError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
