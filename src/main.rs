use clap::{crate_version, App, Arg};

const OLD_PATTERN_PARAMETER_NAME: &str = "old pattern";
const NEW_PATTERN_PARAMETER_NAME: &str = "new pattern";

fn main() {
    let matches = App::new("mvr")
        .author("Jeremy Stucki")
        .version(crate_version!())
        .about("Rename batches of files")
        .arg(Arg::with_name(OLD_PATTERN_PARAMETER_NAME).required(true))
        .arg(Arg::with_name(NEW_PATTERN_PARAMETER_NAME).required(true))
        .get_matches();
}
