use std::error::Error;
use std::fmt;
use std::fmt::Display;

const WILDCARD_TOKEN: char = '*';
const FIXED_LENGTH_TOKEN: char = '?';

#[derive(Debug)]
enum Token {
    Text(String),
    FixedLength(usize),
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
        let elements = get_search_pattern_elements(pattern)?
            .into_iter()
            .map(|element| SearchPatternElement {
                tokens: vec![element],
                is_matching_group: false,
            })
            .collect();

        Ok(Self { elements })
    }
}

fn get_search_pattern_elements(pattern: &str) -> Result<Vec<Token>, PatternError> {
    let first_character = pattern
        .chars()
        .next()
        .ok_or_else(|| PatternError::EmptyPattern)?;

    let end_of_token = match first_character {
        WILDCARD_TOKEN => Some(1),
        FIXED_LENGTH_TOKEN => pattern.chars().position(|c| c != FIXED_LENGTH_TOKEN),
        _ => pattern
            .chars()
            .position(|c| c == WILDCARD_TOKEN || c == FIXED_LENGTH_TOKEN),
    }
    .unwrap_or(pattern.len());

    let token = match first_character {
        WILDCARD_TOKEN => Token::Wildcard,
        FIXED_LENGTH_TOKEN => Token::FixedLength(end_of_token),
        _ => Token::Text(pattern[..end_of_token].to_string()),
    };

    let mut tokens = vec![token];

    if pattern.len() != end_of_token {
        tokens.append(&mut get_search_pattern_elements(&pattern[end_of_token..])?);
    }

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
