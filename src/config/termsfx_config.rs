use serde::Deserialize;
use std::fmt::Debug;

#[derive(Debug, Deserialize, Clone)]
pub struct TermsfxConfig {
    pub commands: Vec<TermsfxCommandConfig>,
}

impl TermsfxConfig {
    pub fn default() -> Self {
        Self {
            commands: vec![],
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        for command in &self.commands {
            command.validate()?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TermsfxCommandConfig {
    pub command: String,
    pub audio_file_path: Option<String>,
    pub audio_file_paths: Option<Vec<String>>,
}

impl TermsfxCommandConfig {
    pub fn new(command: String, audio_file_path: Option<String>, audio_file_paths: Option<Vec<String>>) -> Self {
        Self {
            command,
            audio_file_path,
            audio_file_paths
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.command.is_empty() {
            return Err("Command cannot be empty".to_string());
        }

        if self.audio_file_path.is_none() && self.audio_file_paths.is_none() {
            return Err("Either audioFilePath or audioFilePaths must be provided".to_string());
        }

        if self.audio_file_path.is_some() && self.audio_file_paths.is_some() {
            return Err("Only one of audioFilePath or audioFilePaths must be provided".to_string());
        }

        Ok(())
    }
}
