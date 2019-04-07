use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::num::NonZeroUsize;

#[derive(Debug)]
enum Token {
    Text(String),
    FixedLength(NonZeroUsize),
    Wildcard,
}

#[derive(Debug)]
struct SearchPatternElement {
    tokens: Vec<Token>,
    is_matching_group: bool,
}

#[derive(Debug)]
enum PatternError {
    InvalidSyntax,
}

impl Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            PatternError::InvalidSyntax => "The given pattern is not valid",
        };

        write!(f, "{}", message)
    }
}

impl Error for PatternError {}

#[derive(Debug)]
struct CaptureGroup {
    contents: String,
}

#[derive(Debug)]
enum MatchingError {
    NotApplicable,
}

impl Display for MatchingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            MatchingError::NotApplicable => "This pattern did not match against the given string",
        };

        write!(f, "{}", message)
    }
}

impl Error for MatchingError {}

trait SearchPattern {
    fn match_against(&self, query: &str) -> Result<Vec<CaptureGroup>, MatchingError>;
}

#[derive(Debug)]
struct SearchPatternImpl {
    elements: Vec<SearchPatternElement>,
}

impl SearchPatternImpl {
    fn try_new(pattern: &str) -> Result<Self, PatternError> {
        unimplemented!()
    }
}

impl SearchPattern for SearchPatternImpl {
    fn match_against(&self, query: &str) -> Result<Vec<CaptureGroup>, MatchingError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_with_text_tokens_only_is_valid() {
        SearchPatternImpl::try_new("foo.bar").unwrap();
    }

    #[test]
    fn pattern_with_wildcard_token_is_valid() {
        SearchPatternImpl::try_new("foo*.bar").unwrap();
    }

    #[test]
    fn pattern_with_wildcard_token_inside_capture_group_is_valid() {
        SearchPatternImpl::try_new("foo(*).bar").unwrap();
    }

    #[test]
    fn pattern_with_fixed_length_token_is_valid() {
        SearchPatternImpl::try_new("foo??.bar").unwrap();
    }

    #[test]
    fn pattern_with_fixed_length_token_inside_capture_group_is_valid() {
        SearchPatternImpl::try_new("foo(??).bar").unwrap();
    }

    #[test]
    fn pattern_with_mismatched_braces_is_invalid() {
        SearchPatternImpl::try_new("foo(.bar").unwrap_err();
    }
}
