use crate::config_provider::{ConfigProvider, OverrideAction};
use crate::matcher::Matcher;
use crate::name_generator::{NameGenerator, NameGeneratorError};
use std::error::Error;
use std::fmt::Display;
use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};
use std::{fmt, io};

#[derive(Debug)]
pub(crate) enum PlannerError {
    Aborted,
    IoError(io::Error),
    MatcherError,
    NameGeneratorError(NameGeneratorError),
    InvalidFileName,
}

impl Error for PlannerError {}

impl Display for PlannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            PlannerError::IoError(_) => "An io error occurred",
            PlannerError::MatcherError => "Could not match name against file",
            PlannerError::NameGeneratorError(_) => "Unable to create the new file name",
            PlannerError::InvalidFileName => "Invalid file name. Make sure it is is valid unicode",
            PlannerError::Aborted => "TODO",
        };

        write!(f, "{}", message)
    }
}

pub(crate) enum PlannedAction {
    Rename(PathBuf, PathBuf),
    Override(PathBuf, PathBuf),
    Ignore(PathBuf),
}

#[cfg_attr(test, mockable)]
pub(crate) trait Planner {
    fn get_actions(&self, directory: PathBuf) -> Result<Vec<PlannedAction>, PlannerError>;
}

#[derive(Debug)]
pub(crate) struct PlannerImpl {
    matcher: Box<dyn Matcher>,
    name_generator: Box<dyn NameGenerator>,
    config_provider: Box<dyn ConfigProvider>,
}

impl PlannerImpl {
    pub(crate) fn new(
        matcher: Box<dyn Matcher>,
        name_generator: Box<dyn NameGenerator>,
        config_provider: Box<dyn ConfigProvider>,
    ) -> Self {
        Self {
            matcher,
            name_generator,
            config_provider,
        }
    }
}

impl Planner for PlannerImpl {
    fn get_actions(&self, directory: PathBuf) -> Result<Vec<PlannedAction>, PlannerError> {
        read_dir(directory)
            .map_err(PlannerError::IoError)?
            .into_iter()
            .map(|entry| entry.map_err(PlannerError::IoError))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .filter(|entry| entry.path().is_file())
            .map(|entry| self.get_action(entry.path()))
            .collect()
    }
}

impl PlannerImpl {
    fn create_new_name(&self, old_name: &str) -> Result<String, PlannerError> {
        let capture_groups = self
            .matcher
            .match_against(old_name)
            .map_err(|_| PlannerError::MatcherError)?;

        self.name_generator
            .generate_name(capture_groups)
            .map_err(PlannerError::NameGeneratorError)
    }

    fn get_action(&self, old_path: PathBuf) -> Result<PlannedAction, PlannerError> {
        let old_name = old_path
            .file_name()
            .unwrap()
            .to_str()
            .ok_or(PlannerError::InvalidFileName)?;

        let new_name = self.create_new_name(old_name)?;

        let new_path = Path::new(&new_name).to_owned();
        if new_path.exists() {
            match self.config_provider.get_override_action(new_path.clone()) {
                OverrideAction::Abort => Err(PlannerError::Aborted),
                OverrideAction::Skip => Ok(PlannedAction::Ignore(old_path)),
                OverrideAction::Override => {
                    Ok(PlannedAction::Override(old_path, new_path.to_owned()))
                }
            }
        } else {
            Ok(PlannedAction::Rename(old_path, new_path.to_owned()))
        }
    }
}
