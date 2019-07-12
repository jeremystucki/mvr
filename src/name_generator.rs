use crate::matcher::CaptureGroup;
use crate::replacement_pattern::{Element, Pattern};
use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum NameGeneratorError {
    MissingCaptureGroup(usize),
}

impl Display for NameGeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            NameGeneratorError::MissingCaptureGroup(capture_group) => {
                format!("Missing capture group: {:?}", capture_group)
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
        self.replacement_pattern
            .elements
            .iter()
            .map(|token| match token {
                Element::Text(text) => Ok(text.clone()),
                Element::CaptureGroup(index) => capture_groups
                    .get(*index)
                    .map(|capture_group| capture_group.contents.clone())
                    .ok_or(NameGeneratorError::MissingCaptureGroup(*index)),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matcher::CaptureGroup;
    use crate::replacement_pattern::Element;

    #[test]
    fn works_with_text_only_pattern() {
        let pattern = Pattern {
            elements: vec![Element::Text(String::from("foo"))],
        };

        let name_generator = NameGeneratorImpl::new(pattern);
        let name = name_generator.generate_name(vec![]);

        assert_eq!("foo", name.unwrap());
    }

    #[test]
    fn fails_with_missing_capture_group() {
        let pattern = Pattern {
            elements: vec![Element::CaptureGroup(2), Element::Text(String::from("foo"))],
        };

        let name_generator = NameGeneratorImpl::new(pattern);
        let name = name_generator.generate_name(vec![]);

        assert_eq!(
            NameGeneratorError::MissingCaptureGroup(2),
            name.unwrap_err()
        );
    }

    #[test]
    fn works_with_single_capture_group() {
        let pattern = Pattern {
            elements: vec![Element::CaptureGroup(1), Element::Text(String::from("foo"))],
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
                Element::CaptureGroup(0),
                Element::Text(String::from("foo")),
                Element::CaptureGroup(1),
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
            elements: vec![Element::CaptureGroup(0)],
        };

        let name_generator = NameGeneratorImpl::new(pattern);
        let name = name_generator.generate_name(vec![CaptureGroup {
            contents: String::from("foo"),
        }]);

        assert_eq!(String::from("foo"), name.unwrap());
    }
}
