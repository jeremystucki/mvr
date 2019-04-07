use clap::{crate_version, App, Arg};

mod pattern;

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
}
