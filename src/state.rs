use std::path::Path;
use std::sync::Arc;
use druid::{Data, Lens};

use crate::config::Config;
use crate::tab::{TabConfig, DynamicTabData};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    config: Config,
    tab_config: TabConfig,
    advanced: DynamicTabData,
    file_path: String,
    content: Arc<String>
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            config: Config::new(),
            tab_config: Default::default(),
            advanced: DynamicTabData::new(1),
            file_path: String::from(""),
            content: Arc::new(String::from("")),
        }
    }

    pub fn update_file_path(&mut self, file_path: String) {
        self.file_path = file_path;
    }

    pub fn open_file<P: AsRef<Path>>(&mut self, file_path: P) -> bool {
        match std::fs::read_to_string(file_path) {
            Ok(content) => {
                self.content = Arc::new(content);
                return true;
            }
            Err(e) => {
                println!("Error opening file: {e}");
                return false;
            }
        }
    }
}