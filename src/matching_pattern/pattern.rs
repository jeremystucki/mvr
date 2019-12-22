use either::Either;
use itertools::Itertools;
use std::fmt::Debug;
use std::iter;
use std::iter::repeat;
use std::num::NonZeroUsize;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token {
    Text(String),
    FixedLength(NonZeroUsize),
    Wildcard,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Element {
    Token(Token),
    Group(Vec<Token>),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Pattern {
    pub(crate) elements: Vec<Element>,
}

impl Pattern {
    pub fn as_glob(&self) -> String {
        self.tokens()
            .map(|element| match element {
                Token::Text(text) => text.clone(),
                Token::FixedLength(length) => repeat('?').take(length.get()).collect(),
                Token::Wildcard => String::from("*"),
            })
            .collect()
    }

    fn tokens(&self) -> impl Iterator<Item = &Token> {
        self.elements.iter().flat_map(|element| match element {
            Element::Token(token) => Either::Left(iter::once(token)),
            Element::Group(tokens) => Either::Right(tokens.iter()),
        })
    }
}

pub(crate) fn contains_repeated_wildcards(pattern: &Pattern) -> bool {
    pattern
        .tokens()
        .filter(|token| match token {
            Token::FixedLength(_) => false,
            _ => true,
        })
        .tuple_windows()
        .any(|(first_value, second_value)| {
            *first_value == Token::Wildcard && *second_value == Token::Wildcard
        })
}
