
use druid::Data;
use serde::{Deserialize, Serialize};

#[derive(Data, Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub font_family: String,
    pub font_size: usize,
    pub line_height: f64
}

impl Config {
    pub fn new() -> Self {
        Config {
            font_family: String::from("monospace"),
            font_size: 16,
            line_height: 20.0,
        }
    }
}