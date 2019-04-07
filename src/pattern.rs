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
    EmptyPattern,
}

impl Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            PatternError::InvalidSyntax => "The given pattern is not valid",
            PatternError::EmptyPattern => "The given pattern is empty",
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

fn get_search_pattern_elements(pattern: &str) -> Result<Vec<Token>, PatternError> {
    let mut chars = pattern.chars();

    let first_character = chars
        .next()
        .into_result()
        .map_err(PatternError::EmptyPattern)?;

    let tokens = match first_character {
        '*' => vec![Token::Wildcard].append(&mut get_search_pattern_elements(&pattern[1..])?),
        '?' => {
            let end_of_fixed_length_token = chars.position(|character| character != '?');

            match end_of_fixed_length_token {
                None => vec![Token::FixedLength(pattern.len())],
                Some(position) => vec![Token::FixedLength(position)]
                    .append(&mut get_search_pattern_elements(&pattern[position..])?),
            }
        }
        _ => {
            let end_of_text_token =
                chars.position(|character| character == '?' || character == '*');

            match end_of_text_token {
                None => vec![Token::Text(first_character.to_string())],
                Some(position) => vec![Token::Text(pattern[..position].to_string())]
                    .append(&mut get_search_pattern_elements(&pattern[position..])?),
            }
        }
    };

    Ok(tokens)
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

    #[test]
    fn empty_pattern_is_invalid() {
        SearchPatternImpl::try_new("").unwrap_err();
    }
}
