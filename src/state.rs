use std::sync::Arc;
use druid::{Data, Lens};

use crate::config::Config;
use crate::tab::{TabConfig, DynamicTabData};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub config: Config,
    pub tab_config: TabConfig,
    pub advanced: DynamicTabData,
    pub multi: Arc<String>
}