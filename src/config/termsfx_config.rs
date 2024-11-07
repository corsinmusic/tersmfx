use std::fmt::Debug;
use std::path::{Path, PathBuf};

use regex::Regex;

use super::termsfx_config_source::{TermsfxCommandConfigSource, TermsfxConfigSource, TermsfxConfigSourceError};


#[derive(Debug, Clone)]
pub struct TermsfxConfig {
    pub commands: Vec<TermsfxCommandConfig>,
}

impl TermsfxConfig {
    pub fn default() -> Self {
        Self {
            commands: vec![],
        }
    }

    pub fn try_from_source<P: AsRef<Path>>(source: TermsfxConfigSource, base_path: P,) -> Result<Self, TermsfxConfigSourceError> {
        let _ = source.validate();

        let commands = source.commands.into_iter().map(|command| {
            TermsfxCommandConfig::try_from_source(command, base_path.as_ref())
        }).collect::<Result<Vec<_>, TermsfxConfigSourceError>>();

        Ok(TermsfxConfig {
            commands: commands?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TermsfxCommandConfig {
    pub command: Regex,
    pub audio_file_path: Option<PathBuf>,
    pub audio_file_paths: Option<Vec<PathBuf>>,
}

impl TermsfxCommandConfig {
    pub fn try_from_source<P: AsRef<Path>>(
        source: TermsfxCommandConfigSource,
        base_path: P,
    ) -> Result<Self, TermsfxConfigSourceError> {
        let command = Regex::new(&source.command)?;

        let base_path = base_path.as_ref();

        let audio_file_path = source.audio_file_path.map(|path_str| {
            let path = PathBuf::from(path_str);
            if path.is_relative() {
                base_path.join(path)
            } else {
                path
            }
        });

        let audio_file_paths = source.audio_file_paths.map(|paths| {
            paths
                .into_iter()
                .map(|path_str| {
                    let path = PathBuf::from(path_str);
                    if path.is_relative() {
                        Ok(base_path.join(path))
                    } else {
                        Ok(path)
                    }
                })
                .collect::<Result<Vec<_>, TermsfxConfigSourceError>>()
        }).transpose()?;

        Ok(TermsfxCommandConfig {
            command,
            audio_file_path,
            audio_file_paths,
        })
    }
}
