use crate::matching_pattern::*;
use std::num::NonZeroUsize;

#[derive(Debug, PartialEq)]
pub struct CaptureGroup {
    pub contents: String,
}

pub trait Matcher {
    fn match_against(&self, input: &str) -> Result<Vec<CaptureGroup>, ()>;
}

pub struct MatcherImpl {
    pattern: Pattern,
}

impl MatcherImpl {
    fn new(pattern: Pattern) -> Self {
        Self { pattern }
    }
}

impl Matcher for MatcherImpl {
    fn match_against(&self, input: &str) -> Result<Vec<CaptureGroup>, ()> {
        let tokens_to_capture_groups: Vec<(&Token, bool)> = self
            .pattern
            .elements
            .iter()
            .flat_map(|element| match element {
                Element::Token(token) => vec![(token, false)],
                Element::Group(tokens) => tokens.iter().map(|token| (token, true)).collect(),
            })
            .collect();

        let tokens: Vec<&Token> = tokens_to_capture_groups
            .iter()
            .map(|(token, _)| *token)
            .collect();

        let mut current_token_index = 0;
        let mut current_position = 0;
        let mut lengths = Vec::with_capacity(tokens.len());
        while current_token_index < tokens.len() {
            let length = consume_token(
                &input[current_position..],
                tokens[current_token_index],
                &tokens.get(current_token_index + 1..).unwrap_or(&[]),
            )?;

            current_token_index += 1;
            current_position += length;
            lengths.push(length);
        }

        if current_position != input.len() {
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

fn consume_token(input: &str, head: &Token, tail: &[&Token]) -> Result<usize, ()> {
    match head {
        Token::Text(text) => consume_text_token(text, input),
        Token::FixedLength(length) => consume_fixed_length_token(*length, input),
        Token::Wildcard => consume_wildcard_token(input, tail),
    }
}

fn consume_text_token(text: &String, input: &str) -> Result<usize, ()> {
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
    match tail.get(0) {
        None => Ok(input.len()),
        Some(Token::Text(text)) => input.find(text).ok_or(()),
        Some(Token::FixedLength(length)) => {
            let length = length.get();
            match tail.get(1) {
                None => Ok(input.len() - length),
                Some(Token::Text(text)) => input[length..]
                    .find(text)
                    .map(|position| position - length)
                    .ok_or(()),
                Some(_) => unreachable!(),
            }
        }
        Some(_) => unreachable!(),
    }
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
}
