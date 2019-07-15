use crate::renamer::Renamer;
use crate::{matching_pattern, replacement_pattern};
use nameof::name_of;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::path::Path;

pub(crate) trait Controller: Debug {
    fn rename_files_by_pattern(
        &self,
        matching_pattern: &str,
        replacement_pattern: &str,
        directory: &Path,
    ) -> Result<(), Box<dyn Error>>;
}

pub(crate) type RenamerFactory =
    dyn Fn(matching_pattern::Pattern, replacement_pattern::Pattern) -> Box<dyn Renamer>;

pub(crate) struct ControllerImpl {
    matching_pattern_parser: Box<dyn matching_pattern::Parser>,
    replacement_pattern_parser: Box<dyn replacement_pattern::Parser>,
    renamer_factory: Box<RenamerFactory>,
}

impl ControllerImpl {
    pub(crate) fn new(
        matching_pattern_parser: Box<dyn matching_pattern::Parser>,
        replacement_pattern_parser: Box<dyn replacement_pattern::Parser>,
        renamer_factory: Box<RenamerFactory>,
    ) -> Self {
        Self {
            matching_pattern_parser,
            replacement_pattern_parser,
            renamer_factory,
        }
    }
}

impl Controller for ControllerImpl {
    fn rename_files_by_pattern(
        &self,
        matching_pattern: &str,
        replacement_pattern: &str,
        directory: &Path,
    ) -> Result<(), Box<dyn Error>> {
        let matching_pattern = self.matching_pattern_parser.parse(matching_pattern)?;
        let replacement_pattern = self.replacement_pattern_parser.parse(replacement_pattern)?;

        let renamer = (self.renamer_factory)(matching_pattern, replacement_pattern);

        renamer.rename_files_in_directory(directory)
    }
}

impl Debug for ControllerImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct(name_of!(type ControllerImpl))
            .field(
                name_of!(matching_pattern_parser in ControllerImpl),
                &self.matching_pattern_parser,
            )
            .field(
                name_of!(replacement_pattern_parser in ControllerImpl),
                &self.replacement_pattern_parser,
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renamer::RenamerMock;
    use std::path::PathBuf;

    #[test]
    fn test() {
        let mut matching_pattern_parser = matching_pattern::ParserMock::new();
        let expected_matching_pattern = matching_pattern::Pattern {
            elements: vec![matching_pattern::Element::Group(vec![
                matching_pattern::Token::Wildcard,
            ])],
        };
        matching_pattern_parser
            .expect_parse(|arg| arg.partial_eq("foo"))
            .returns(Ok(expected_matching_pattern.clone()));

        let mut replacement_pattern_parser = replacement_pattern::ParserMock::new();
        let expected_replacement_pattern = replacement_pattern::Pattern {
            elements: vec![replacement_pattern::Element::CaptureGroup(2)],
        };
        replacement_pattern_parser
            .expect_parse(|arg| arg.partial_eq("bar"))
            .returns(Ok(expected_replacement_pattern.clone()));

        let renamer_factory: Box<RenamerFactory> =
            Box::new(move |matching_pattern, replacement_pattern| {
                assert_eq!(expected_matching_pattern, matching_pattern);
                assert_eq!(expected_replacement_pattern, replacement_pattern);

                let mut renamer = RenamerMock::new();
                renamer
                    .expect_rename_files_in_directory(|arg| arg.partial_eq(PathBuf::from("baz")))
                    .returns_once(Ok(()));

                Box::new(renamer)
            });

        let controller = ControllerImpl::new(
            Box::new(matching_pattern_parser),
            Box::new(replacement_pattern_parser),
            renamer_factory,
        );

        controller
            .rename_files_by_pattern("foo", "bar", &PathBuf::from("baz"))
            .unwrap();
    }
}
