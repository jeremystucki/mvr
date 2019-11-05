use crate::config_provider::{CliConfigProvider, CliOptions, OverrideMode};
use crate::controller::{Controller, ControllerImpl, PlannerFactory, RenamerFactory};
use crate::matcher::MatcherImpl;
use crate::name_generator::NameGeneratorImpl;
use crate::planner::PlannerImpl;
use crate::renamer::RenamerImpl;
use clap::{crate_version, App, Arg};
use std::env::current_dir;
use std::error::Error;

mod config_provider;
mod controller;
mod matcher;
mod matching_pattern;
mod name_generator;
mod planner;
mod renamer;
mod replacement_pattern;

const OLD_PATTERN_PARAMETER_NAME: &str = "old pattern";
const NEW_PATTERN_PARAMETER_NAME: &str = "new pattern";

fn main() -> Result<(), Box<dyn Error>> {
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

    let cli_options = CliOptions {
        override_mode: OverrideMode::AbortOnOverride,
    };

    let planner_factory: Box<PlannerFactory> =
        Box::new(move |matching_pattern, replacement_pattern| {
            let matcher = MatcherImpl::new(matching_pattern);
            let name_generator = NameGeneratorImpl::new(replacement_pattern);

            let config_provider = CliConfigProvider {
                cli_options: cli_options.clone(),
            };

            Box::new(PlannerImpl::new(
                Box::new(matcher),
                Box::new(name_generator),
                Box::new(config_provider),
            ))
        });

    let matcher = MatcherImpl::new(matching_pattern);
    let name_generator = NameGeneratorImpl::new(replacement_pattern);

    let renamer = RenamerImpl::new(Box::new(matcher), Box::new(name_generator));

    let controller = ControllerImpl::new(
        Box::new(matching_pattern::ParserImpl::new()),
        Box::new(replacement_pattern::ParserImpl::new()),
        Box::new(renamer),
        planner_factory,
    );

    let matching_pattern = matches.value_of(OLD_PATTERN_PARAMETER_NAME).unwrap();
    let replacement_pattern = matches.value_of(NEW_PATTERN_PARAMETER_NAME).unwrap();
    let directory = current_dir().expect("Cannot access directory");

    controller.rename_files_by_pattern(matching_pattern, replacement_pattern, &directory)
}
