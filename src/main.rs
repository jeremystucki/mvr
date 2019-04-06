use clap::{App, Arg, crate_version};

fn main() {
    let matches = App::new("mvr")
        .author("Jeremy Stucki")
        .version(crate_version!())
        .about("Rename batches of files")
        .arg(Arg::with_name("old pattern").required(true))
        .arg(Arg::with_name("new pattern").required(true))
        .get_matches();
}
