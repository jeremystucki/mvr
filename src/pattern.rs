use std::num::NonZeroUsize;

enum Token {
    Text(String),
    FixedLength(NonZeroUsize),
    Wildcard,
}

struct SearchPatternElement {
    tokens: Vec<Token>,
    is_matching_group: bool,
}

enum PatternError {
    InvalidSyntax(String),
}

struct MatchingGroup {
    contents: String,
}

enum SearchPatternError {
    NotApplicable,
}

trait SearchPattern {
    fn match_against(&self, query: &str) -> Result<Vec<MatchingGroup>, SearchPatternError>;
}

struct SearchPatternImpl {
    elements: Vec<SearchPatternElement>,
}

impl SearchPatternImpl {
    fn try_new() -> Result<Self, PatternError> {
        unimplemented!()
    }
}

impl SearchPattern for SearchPatternImpl {
    fn match_against(&self, query: &str) -> Result<Vec<MatchingGroup>, SearchPatternError> {
        unimplemented!()
    }
}
