#[derive(Debug)]
struct CaptureGroup {
    contents: String,
}

trait Matcher {
    fn match_against(&self, input: &str) -> Option<Vec<CaptureGroup>>;
}

struct MatcherImpl {}
