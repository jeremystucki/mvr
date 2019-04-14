#[derive(Debug)]
pub struct CaptureGroup {
    contents: String,
}

pub trait Matcher {
    fn match_against(&self, input: &str) -> Option<Vec<CaptureGroup>>;
}

pub struct MatcherImpl {}
