use crate::matcher::MatcherImpl;
use crate::matching_pattern::Parser as MatchingPatternParser;
use crate::name_generator::NameGeneratorImpl;
use crate::renamer::{Renamer, RenamerImpl};
use crate::replacement_pattern::Parser as ReplacementPatternParser;
use clap::{crate_version, App, Arg};
use std::env::current_dir;

mod matcher;
mod matching_pattern;
mod name_generator;
mod renamer;
mod replacement_pattern;

const OLD_PATTERN_PARAMETER_NAME: &str = "old pattern";
const NEW_PATTERN_PARAMETER_NAME: &str = "new pattern";

fn main() {
    let matches = App::new("mvr")
        .author("Jeremy Stucki")
        .version(crate_version!())
        .about("Rename batches of files")
        .arg(
            Arg::with_name(OLD_PATTERN_PARAMETER_NAME)
                .required(true)
                .help(
                    "Use braces to indicate groups\n\
                     Use question marks to match a single character\n\
                     Use asterisks to match any amount of characters",
                ),
        )
        .arg(
            Arg::with_name(NEW_PATTERN_PARAMETER_NAME)
                .required(true)
                .help("Use $n to insert a matched group (0-based)"),
        )
        .get_matches();

    let matching_pattern =
        parse_matching_pattern(matches.value_of(OLD_PATTERN_PARAMETER_NAME).unwrap());

    let replacement_pattern =
        parse_replacement_pattern(matches.value_of(NEW_PATTERN_PARAMETER_NAME).unwrap());

    let matcher = MatcherImpl::new(matching_pattern);
    let name_generator = NameGeneratorImpl::new(replacement_pattern);

    let renamer = RenamerImpl::new(Box::new(matcher), Box::new(name_generator));

    let directory = current_dir().expect("Cannot access directory");
    renamer
        .rename_files_in_directory(directory.as_path())
        .unwrap();
}

fn parse_matching_pattern(string: &str) -> matching_pattern::Pattern {
    let parser = matching_pattern::ParserImpl::new();
    match parser.parse(string) {
        Ok(pattern) => pattern,
        Err(matching_pattern::ParsingError::InvalidSyntax) => panic!("Invalid matching pattern"),
    }
}

fn parse_replacement_pattern(string: &str) -> replacement_pattern::Pattern {
    let parser = replacement_pattern::ParserImpl::new();
    match parser.parse(string) {
        Ok(pattern) => pattern,
        Err(replacement_pattern::ParsingError::InvalidSyntax) => {
            panic!("Invalid replacement pattern")
        }
    }
}
