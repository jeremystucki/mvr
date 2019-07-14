use crate::matcher::{Matcher, MatcherImpl};
use crate::matching_pattern::Parser as MatchingPatternParser;
use crate::name_generator::{NameGenerator, NameGeneratorImpl};
use crate::replacement_pattern::Parser as ReplacementPatternParser;
use clap::{crate_version, App, Arg};
use std::env::current_dir;
use std::fs::{read_dir, rename};
use std::io;
use std::path::PathBuf;

mod matcher;
mod matching_pattern;
mod name_generator;
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

    let directory = current_dir().unwrap();
    rename_files(&directory, &matcher, &name_generator).unwrap();
}

fn parse_matching_pattern(string: &str) -> matching_pattern::Pattern {
    let parser = matching_pattern::ParserImpl::new();
    match parser.parse(string) {
        Ok(pattern) => pattern,
        Err(_) => unimplemented!(),
    }
}

fn parse_replacement_pattern(string: &str) -> replacement_pattern::Pattern {
    let parser = replacement_pattern::ParserImpl::new();
    match parser.parse(string) {
        Ok(pattern) => pattern,
        Err(_) => unimplemented!(),
    }
}

fn rename_files(
    directory: &PathBuf,
    matcher: &dyn Matcher,
    name_generator: &dyn NameGenerator,
) -> Result<(), io::Error> {
    for entry in read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let old_name = entry.file_name();
        let new_name = create_new_name(&old_name.to_str().unwrap(), matcher, name_generator);

        rename(old_name, new_name).unwrap();
    }

    Ok(())
}

fn create_new_name(
    old_name: &str,
    matcher: &dyn Matcher,
    name_generator: &dyn NameGenerator,
) -> String {
    let capture_groups = matcher.match_against(old_name).unwrap();
    name_generator.generate_name(capture_groups).unwrap()
}
