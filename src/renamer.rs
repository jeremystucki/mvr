use crate::matcher::Matcher;
use crate::name_generator::{NameGenerator, NameGeneratorError};
use std::error::Error;
use std::fmt::Display;
use std::fs::{read_dir, rename};
use std::path::Path;
use std::{fmt, io};

#[derive(Debug)]
pub(crate) enum RenamerError {
    IoError(io::Error),
    MatcherError,
    NameGeneratorError(NameGeneratorError),
    InvalidFileName,
}

impl Display for RenamerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            RenamerError::IoError(_) => "An io error occurred",
            RenamerError::MatcherError => "Could not match name against file",
            RenamerError::NameGeneratorError(_) => "Unable to create the new file name",
            RenamerError::InvalidFileName => "Invalid file name. Make sure it is is valid unicode",
        };

        write!(f, "{}", message)
    }
}

impl Error for RenamerError {}

pub(crate) trait Renamer {
    fn rename_files_in_directory(&self, directory: &Path) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub(crate) struct RenamerImpl {
    matcher: Box<dyn Matcher>,
    name_generator: Box<dyn NameGenerator>,
}

impl RenamerImpl {
    pub(crate) fn new(matcher: Box<dyn Matcher>, name_generator: Box<dyn NameGenerator>) -> Self {
        Self {
            matcher,
            name_generator,
        }
    }

    fn create_new_name(&self, old_name: &str) -> Result<String, RenamerError> {
        let capture_groups = self
            .matcher
            .match_against(old_name)
            .map_err(|_| RenamerError::MatcherError)?;

        self.name_generator
            .generate_name(capture_groups)
            .map_err(RenamerError::NameGeneratorError)
    }
}

impl Renamer for RenamerImpl {
    fn rename_files_in_directory(&self, directory: &Path) -> Result<(), Box<dyn Error>> {
        let dir_entry = read_dir(directory).map_err(RenamerError::IoError)?;

        for entry in dir_entry {
            let entry = entry.map_err(RenamerError::IoError)?;

            let path = entry.path();
            if path.is_dir() {
                continue;
            }

            let old_name = entry.file_name();

            let new_name = match self
                .create_new_name(old_name.to_str().ok_or(RenamerError::InvalidFileName)?)
            {
                Ok(new_name) => new_name,
                Err(_) => {
                    eprintln!("Ignoring file: {:?}", old_name);
                    continue;
                }
            };

            println!("Renaming {:?} → {:?}", &old_name, &new_name);

            if Path::new(&new_name).exists() {
                eprintln!("Path already exists. Skipping...");
            }

            rename(old_name, new_name).map_err(RenamerError::IoError)?;
        }

        Ok(())
    }
}
