use crate::matching_pattern::*;
use std::fmt::Debug;
use std::num::NonZeroUsize;

#[derive(Debug, PartialEq)]
pub(crate) struct CaptureGroup {
    pub(crate) contents: String,
}

pub(crate) trait Matcher: Debug {
    fn match_against(&self, input: &str) -> Result<Vec<CaptureGroup>, ()>;
}

#[derive(Debug)]
pub(crate) struct MatcherImpl {
    pattern: Pattern,
}

impl MatcherImpl {
    pub(crate) fn new(pattern: Pattern) -> Self {
        Self { pattern }
    }
}

impl Matcher for MatcherImpl {
    fn match_against(&self, input: &str) -> Result<Vec<CaptureGroup>, ()> {
        let tokens_to_capture_groups: Vec<_> = self
            .pattern
            .elements
            .iter()
            .flat_map(|element| match element {
                Element::Token(token) => vec![(token, false)],
                Element::Group(tokens) => tokens.iter().map(|token| (token, true)).collect(),
            })
            .collect();

        let tokens: Vec<_> = tokens_to_capture_groups
            .iter()
            .map(|(token, _)| *token)
            .collect();

        let lengths = consume_tokens(input, &tokens).collect::<Result<Vec<_>, _>>()?;

        let lengths_sum: usize = lengths.iter().sum();
        if lengths_sum != input.len() {
            return Err(());
        }

        let positions_and_lengths = lengths.iter().scan(0 as usize, |position, length| {
            let own_position = *position;
            *position += *length;
            Some((own_position, *length))
        });

        Ok(tokens_to_capture_groups
            .iter()
            .zip(positions_and_lengths)
            .filter(|((_, capture_group), _)| *capture_group)
            .map(|((_, _), (position, length))| CaptureGroup {
                contents: String::from(&input[position..position + length]),
            })
            .collect())
    }
}

fn consume_tokens<'a>(
    input: &'a str,
    tokens: &'a [&'a Token],
) -> impl Iterator<Item = Result<usize, ()>> + 'a {
    let mut current_position = 0;
    tokens.iter().enumerate().map(move |(token_index, token)| {
        let length = consume_token(
            &input[current_position..],
            token,
            &tokens.get(token_index + 1..).unwrap_or(&[]),
        )?;

        current_position += length;

        Ok(length)
    })
}

fn consume_token(input: &str, head: &Token, tail: &[&Token]) -> Result<usize, ()> {
    match head {
        Token::Text(text) => consume_text_token(text, input),
        Token::FixedLength(length) => consume_fixed_length_token(*length, input),
        Token::Wildcard => consume_wildcard_token(input, tail),
    }
}

fn consume_text_token(text: &str, input: &str) -> Result<usize, ()> {
    if input.starts_with(text) {
        Ok(text.len())
    } else {
        Err(())
    }
}

fn consume_fixed_length_token(length: NonZeroUsize, input: &str) -> Result<usize, ()> {
    let length = length.get();

    if input.len() >= length {
        Ok(length)
    } else {
        Err(())
    }
}

fn consume_wildcard_token(input: &str, tail: &[&Token]) -> Result<usize, ()> {
    if tail.is_empty() {
        return Ok(input.len());
    }

    for (char_index, _) in input.char_indices() {
        if consume_tokens(&input[char_index..], tail).all(|result| result.is_ok()) {
            return Ok(char_index);
        }
    }

    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_without_group_with_exact_match() {
        let expected = Ok(vec![]);

        let pattern = Pattern {
            elements: vec![Element::Token(Token::Text(String::from("foo")))],
        };

        let matcher = MatcherImpl::new(pattern);

        let actual = matcher.match_against("foo");

        assert_eq!(expected, actual)
    }

    #[test]
    fn parse_without_group_without_exact_match() {
        let expected = Err(());

        let pattern = Pattern {
            elements: vec![Element::Token(Token::Text(String::from("foo")))],
        };

        let matcher = MatcherImpl::new(pattern);

        let actual = matcher.match_against("foo.bar");

        assert_eq!(expected, actual)
    }

    #[test]
    fn parse_with_fixed_length() {
        let expected = Ok(vec![]);

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
        let expected = Ok(vec![CaptureGroup {
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
        let expected = Ok(vec![]);

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
        let expected = Ok(vec![CaptureGroup {
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
        let expected = Ok(vec![
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

    #[test]
    fn wildcard_looks_ahead_for_all_following_tokens() {
        let expected = Ok(vec![CaptureGroup {
            contents: String::from("baz"),
        }]);

        let pattern = Pattern {
            elements: vec![
                Element::Token(Token::Wildcard),
                Element::Token(Token::Text(String::from("."))),
                Element::Group(vec![Token::Text(String::from("baz"))]),
            ],
        };

        let matcher = MatcherImpl::new(pattern);

        let actual = matcher.match_against("foo.bar.baz");

        assert_eq!(expected, actual)
    }

    #[test]
    fn capture_groups_work() {
        let expected = Ok(vec![CaptureGroup {
            contents: String::from("foo.bar"),
        }]);

        let pattern = Pattern {
            elements: vec![
                Element::Group(vec![Token::Wildcard, Token::Text(String::from(".bar"))]),
                Element::Token(Token::Text(String::from(".bar"))),
            ],
        };

        let matcher = MatcherImpl::new(pattern);

        let actual = matcher.match_against("foo.bar.bar");

        assert_eq!(expected, actual)
    }

    #[test]
    fn fails_if_input_is_too_short() {
        let expected = Err(());

        let pattern = Pattern {
            elements: vec![Element::Token(Token::FixedLength(
                NonZeroUsize::new(5).unwrap(),
            ))],
        };

        let matcher = MatcherImpl::new(pattern);

        let actual = matcher.match_against("foo");

        assert_eq!(expected, actual)
    }
}
