use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct OpenHandler {
    pub id: String,
    pub display_name: String,
    pub program: String,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
    pub cwd: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub open_handlers: Vec<OpenHandler>,
    pub default_open_handler: Option<String>
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct OpenPosition {
    pub file: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
}