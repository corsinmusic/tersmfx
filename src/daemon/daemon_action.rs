use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DaemonAction {
    Play(String),
    PrintConfig,
}
