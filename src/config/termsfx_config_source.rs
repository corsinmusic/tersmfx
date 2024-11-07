use serde::Deserialize;
use std::fmt::Debug;
use thiserror::Error;

use super::termsfx_config::TermsfxCommandConfig;

#[derive(Debug, Error)]
pub enum TermsfxConfigSourceError {
    #[error("Invalid regex pattern: {0}")]
    InvalidRegex(#[from] regex::Error),
    #[error("Invalid path: {0}")]
    InvalidPath(#[from] std::io::Error),
    #[error("Invalid config: {0}")]
    InvalidConfig(&'static str),
}

#[derive(Debug, Deserialize, Clone)]
pub struct TermsfxConfigSource {
    pub commands: Vec<TermsfxCommandConfigSource>,
}

impl TermsfxConfigSource {
    pub fn validate(&self) -> Result<(), TermsfxCommandConfig> {
        for command in &self.commands {
            let _ = command.validate();
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TermsfxCommandConfigSource {
    pub command: String,
    pub audio_file_path: Option<String>,
    pub audio_file_paths: Option<Vec<String>>,
}

impl TermsfxCommandConfigSource {
    pub fn validate(&self) -> Result<(), TermsfxConfigSourceError> {
        if self.command.is_empty() {
            return Err(TermsfxConfigSourceError::InvalidConfig("command must be provided"));
        }

        if self.audio_file_path.is_none() && self.audio_file_paths.is_none() {
            return Err(TermsfxConfigSourceError::InvalidConfig("audioFilePath or audioFilePaths must be provided"));
        }

        if self.audio_file_path.is_some() && self.audio_file_paths.is_some() {
            return Err(TermsfxConfigSourceError::InvalidConfig("audioFilePath and audioFilePaths cannot both be provided"));
        }

        Ok(())
    }
}
