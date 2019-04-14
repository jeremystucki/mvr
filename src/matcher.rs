use crate::matching_pattern::*;
use std::num::NonZeroUsize;

#[derive(Debug, PartialEq)]
struct CaptureGroup {
    contents: String,
}

trait Matcher {
    fn match_against(&self, input: &str) -> Option<Vec<CaptureGroup>>;
}

struct MatcherImpl {
    pattern: Pattern,
}

impl MatcherImpl {
    fn new(pattern: Pattern) -> Self {
        Self { pattern }
    }
}

impl Matcher for MatcherImpl {
    fn match_against(&self, input: &str) -> Option<Vec<CaptureGroup>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_without_group_with_exact_match() {
        let expected = Some(vec![]);

        let pattern = Pattern {
            elements: vec![Element::Token(Token::Text(String::from("foo")))],
        };

        let matcher = MatcherImpl::new(pattern);

        let actual = matcher.match_against("foo");

        assert_eq!(expected, actual)
    }

    #[test]
    fn parse_without_group_without_exact_match() {
        let expected = None;

        let pattern = Pattern {
            elements: vec![Element::Token(Token::Text(String::from("foo")))],
        };

        let matcher = MatcherImpl::new(pattern);

        let actual = matcher.match_against("foo.bar");

        assert_eq!(expected, actual)
    }

    #[test]
    fn parse_with_fixed_length() {
        let expected = Some(vec![]);

        let pattern = Pattern {
            elements: vec![
                Element::Token(Token::Text(String::from("f"))),
                Element::Token(Token::FixedLength(NonZeroUsize::new(2).unwrap())),
            ],
        };

        let matcher = MatcherImpl::new(pattern);

        let actual = matcher.match_against("foo");

        assert_eq!(expected, actual)
    }

    #[test]
    fn parse_with_fixed_length_inside_group() {
        let expected = Some(vec![CaptureGroup {
            contents: String::from("oo"),
        }]);

        let pattern = Pattern {
            elements: vec![
                Element::Token(Token::Text(String::from("f"))),
                Element::Group(vec![Token::FixedLength(NonZeroUsize::new(2).unwrap())]),
            ],
        };

        let matcher = MatcherImpl::new(pattern);

        let actual = matcher.match_against("foo");

        assert_eq!(expected, actual)
    }

    #[test]
    fn parse_with_wildcard() {
        let expected = Some(vec![]);

        let pattern = Pattern {
            elements: vec![
                Element::Token(Token::Text(String::from("f"))),
                Element::Token(Token::Wildcard),
                Element::Token(Token::Text(String::from(".bar"))),
            ],
        };

        let matcher = MatcherImpl::new(pattern);

        let actual = matcher.match_against("foo.bar");

        assert_eq!(expected, actual)
    }

    #[test]
    fn parse_with_wildcard_inside_group() {
        let expected = Some(vec![CaptureGroup {
            contents: String::from("oo"),
        }]);

        let pattern = Pattern {
            elements: vec![
                Element::Token(Token::Text(String::from("f"))),
                Element::Group(vec![Token::Wildcard]),
                Element::Token(Token::Text(String::from(".bar"))),
            ],
        };

        let matcher = MatcherImpl::new(pattern);

        let actual = matcher.match_against("foo.bar");

        assert_eq!(expected, actual)
    }

    #[test]
    fn parse_with_multiple_groups() {
        let expected = Some(vec![
            CaptureGroup {
                contents: String::from("oo"),
            },
            CaptureGroup {
                contents: String::from("bar"),
            },
        ]);

        let pattern = Pattern {
            elements: vec![
                Element::Token(Token::Text(String::from("f"))),
                Element::Group(vec![Token::FixedLength(NonZeroUsize::new(2).unwrap())]),
                Element::Token(Token::Text(String::from("."))),
                Element::Group(vec![Token::Wildcard]),
            ],
        };

        let matcher = MatcherImpl::new(pattern);

        let actual = matcher.match_against("foo.bar");

        assert_eq!(expected, actual)
    }
}
