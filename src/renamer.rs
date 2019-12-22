use crate::matcher::Matcher;
use crate::name_generator::{NameGenerator, NameGeneratorError};
use glob::glob;
use std::error::Error;
use std::fmt::Display;
use std::fs::{create_dir_all, rename};
use std::path::Path;
use std::{fmt, io};

#[cfg(test)]
use mockiato::mockable;
use std::env::set_current_dir;

#[derive(Debug)]
pub(crate) enum RenamerError {
    IoError(io::Error),
    MatcherError,
    NameGeneratorError(NameGeneratorError),
    InvalidFileName,
    InternalError(Box<dyn Error>),
}

impl Display for RenamerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            RenamerError::IoError(_) => "An io error occurred",
            RenamerError::MatcherError => "Could not match name against file",
            RenamerError::NameGeneratorError(_) => "Unable to create the new file name",
            RenamerError::InvalidFileName => "Invalid file name. Make sure it is valid unicode",
            RenamerError::InternalError(_) => "An internal error occurred",
        };

        write!(f, "{}", message)
    }
}

impl Error for RenamerError {}

#[cfg_attr(test, mockable)]
pub(crate) trait Renamer {
    fn rename_files_in_directory(&self, directory: &Path) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub(crate) struct RenamerImpl {
    matching_pattern_glob: String,
    matcher: Box<dyn Matcher>,
    name_generator: Box<dyn NameGenerator>,
}

impl RenamerImpl {
    pub(crate) fn new(
        matching_pattern_glob: String,
        matcher: Box<dyn Matcher>,
        name_generator: Box<dyn NameGenerator>,
    ) -> Self {
        Self {
            matching_pattern_glob,
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
        set_current_dir(directory).map_err(|error| RenamerError::InternalError(Box::new(error)))?;

        // TODO: Make sure globs work the same way as the matcher
        for old_path in glob(self.matching_pattern_glob.as_ref())
            .map_err(|error| RenamerError::InternalError(Box::new(error)))?
            .filter_map(Result::ok)
        {
            let old_name = old_path.to_str().ok_or(RenamerError::InvalidFileName)?;

            let new_name = match self.create_new_name(old_name) {
                Ok(new_name) => new_name,
                Err(_) => {
                    eprintln!("Ignoring file: {:?}", old_name);
                    continue;
                }
            };

            println!("Renaming {:?} → {:?}", &old_name, &new_name);

            let new_path = Path::new(&new_name);
            if new_path.exists() {
                eprintln!("Path already exists. Skipping...");
            }

            create_dir_all(new_path.parent().unwrap()).map_err(RenamerError::IoError)?;

            rename(old_path, new_path).map_err(RenamerError::IoError)?;
        }

        Ok(())
    }
}
