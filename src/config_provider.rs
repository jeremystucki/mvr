use std::fmt::Debug;
use std::path::{Path, PathBuf};

pub(crate) enum OverrideAction {
    Abort,
    Skip,
    Override,
}

pub(crate) trait ConfigProvider: Debug {
    fn get_override_action(&self, file: PathBuf) -> OverrideAction;
}

#[derive(Clone, Debug)]
pub(crate) enum OverrideMode {
    OverrideWithoutConfirmation,
    OverrideWithManualConfirmation,
    DoNotOverride,
    AbortOnOverride,
}

#[derive(Clone, Debug)]
pub(crate) struct CliOptions {
    override_mode: OverrideMode,
}

#[derive(Debug)]
pub(crate) struct CliConfigProvider {
    cli_options: CliOptions,
}

impl CliConfigProvider {
    fn new(cli_options: CliOptions) -> Self {
        Self { cli_options }
    }
}

impl ConfigProvider for CliConfigProvider {
    fn get_override_action(&self, file: PathBuf) -> OverrideAction {
        match self.cli_options.override_mode {
            OverrideMode::OverrideWithoutConfirmation => OverrideAction::Override,
            OverrideMode::OverrideWithManualConfirmation => unimplemented!(),
            OverrideMode::DoNotOverride => OverrideAction::Skip,
            OverrideMode::AbortOnOverride => OverrideAction::Abort,
        }
    }
}
