#![allow(deprecated)]

use config::{Config, File};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;
use crate::config::termsfx_config::TermsfxConfig;

use super::termsfx_config_source::TermsfxConfigSource;

pub struct TermsfxConfigLoader {
    config_path: PathBuf,
    config: Arc<RwLock<TermsfxConfig>>,
}

impl TermsfxConfigLoader {
    pub fn new() -> Self {

        Self {
            config_path: dirs::home_dir()
                .expect("Could not find home directory")
                .join(".config")
                .join("termsfx")
                .join("termsfx.json"),
            config: Arc::new(RwLock::new(TermsfxConfig::default())),
        }
    }

    pub fn watch(&self) {
        let config_path = self.config_path.clone();
        let config_clone = self.config.clone();

        // Load initial config
        let initial_config = TermsfxConfigLoader::load_config(&config_path);
        *config_clone.write().unwrap() = initial_config;

        self.print();

        std::thread::spawn(move || {
            let (tx, rx) = channel();

            let mut watcher: RecommendedWatcher = Watcher::new(
                tx,
                notify::Config::default().with_poll_interval(Duration::from_secs(2)),
            )
            .unwrap();

            watcher
                .watch(config_path.parent().unwrap(), RecursiveMode::NonRecursive)
                .unwrap();

            loop {
                match rx.recv() {
                    Ok(Ok(Event {
                        kind: notify::event::EventKind::Modify(_),
                        ..
                    })) => {
                        // Update config
                        let new_config = TermsfxConfigLoader::load_config(&config_path);
                        *config_clone.write().unwrap() = new_config;
                    }
                    Err(e) => println!("watch error: {e:?}"),
                    _ => {}
                }
            }
        });
    }

    pub fn print(&self) {
        println!("{:#?}", self.config.read().unwrap());
    }

    pub fn config(&self) -> TermsfxConfig {
        self.config.read().unwrap().clone()
    }

    fn load_config(config_path: &PathBuf) -> TermsfxConfig {
        let mut settings = Config::default();

        settings.merge(File::from(config_path.clone())).unwrap();

        let source_config = settings.try_deserialize::<TermsfxConfigSource>().unwrap();
        source_config.validate().unwrap();

        TermsfxConfig::try_from_source(source_config, config_path.parent().unwrap()).unwrap()
    }
}

