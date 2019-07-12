use crate::matcher::CaptureGroup;
use crate::replacement_pattern::Pattern;
use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum NameGeneratorError {
    MissingCaptureGroups(Vec<usize>),
}

impl Display for NameGeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            NameGeneratorError::MissingCaptureGroups(capture_groups) => {
                format!("Missing capture groups: {:?}", capture_groups)
            }
        };

        write!(f, "{}", message)
    }
}

impl Error for NameGeneratorError {}

trait NameGenerator {
    fn generate_name(
        &self,
        capture_groups: Vec<CaptureGroup>,
    ) -> Result<String, NameGeneratorError>;
}

struct NameGeneratorImpl {
    replacement_pattern: Pattern,
}

impl NameGeneratorImpl {
    fn new(replacement_pattern: Pattern) -> Self {
        Self {
            replacement_pattern,
        }
    }
}

impl NameGenerator for NameGeneratorImpl {
    fn generate_name(
        &self,
        capture_groups: Vec<CaptureGroup>,
    ) -> Result<String, NameGeneratorError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matcher::CaptureGroup;
    use crate::replacement_pattern::Token;

    #[test]
    fn works_with_text_only_pattern() {
        let pattern = Pattern {
            elements: vec![Token::Text(String::from("foo"))],
        };

        let name_generator = NameGeneratorImpl::new(pattern);
        let name = name_generator.generate_name(vec![]);

        assert_eq!("foo", name.unwrap());
    }

    #[test]
    fn fails_with_missing_capture_group() {
        let pattern = Pattern {
            elements: vec![Token::CaptureGroup(2), Token::Text(String::from("foo"))],
        };

        let name_generator = NameGeneratorImpl::new(pattern);
        let name = name_generator.generate_name(vec![]);

        assert_eq!(
            NameGeneratorError::MissingCaptureGroups(vec![2]),
            name.unwrap_err()
        );
    }

    #[test]
    fn works_with_single_capture_group() {
        let pattern = Pattern {
            elements: vec![Token::CaptureGroup(1), Token::Text(String::from("foo"))],
        };

        let name_generator = NameGeneratorImpl::new(pattern);
        let name = name_generator.generate_name(vec![
            CaptureGroup {
                contents: String::from("foo"),
            },
            CaptureGroup {
                contents: String::from("bar"),
            },
            CaptureGroup {
                contents: String::from("baz"),
            },
        ]);

        assert_eq!(String::from("foobaz"), name.unwrap());
    }

    #[test]
    fn works_with_multiple_capture_groups() {
        let pattern = Pattern {
            elements: vec![
                Token::CaptureGroup(0),
                Token::Text(String::from("foo")),
                Token::CaptureGroup(1),
            ],
        };

        let name_generator = NameGeneratorImpl::new(pattern);
        let name = name_generator.generate_name(vec![
            CaptureGroup {
                contents: String::from("foo"),
            },
            CaptureGroup {
                contents: String::from("bar"),
            },
            CaptureGroup {
                contents: String::from("baz"),
            },
        ]);

        assert_eq!(String::from("foobar"), name.unwrap());
    }

    #[test]
    fn works_with_capture_group_only() {
        let pattern = Pattern {
            elements: vec![Token::CaptureGroup(0)],
        };

        let name_generator = NameGeneratorImpl::new(pattern);
        let name = name_generator.generate_name(vec![CaptureGroup {
            contents: String::from("foo"),
        }]);

        assert_eq!(String::from("foo"), name.unwrap());
    }
}
