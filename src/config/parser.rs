// parse the json config file
use std::fs::File;
use std::io::Read;


use crate::config::datadef::{Config, OpenPosition};

impl Config {
    pub fn from_json_file(file_path: &str) -> Result<Self, String> {
        let mut file = File::open(file_path).map_err(|e| e.to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
        let config: Config = serde_json::from_str(&contents).map_err(|e| e.to_string())?;
        Ok(config)
    }
}

impl OpenPosition {
    // Parse a URL in the format "openwith://<filepath>[?line=<line>[&column=<column>"]]
    pub fn parse(url: &str) -> Result<Self, String> {
        let url = url.trim_start_matches("openwith://");
        let mut parts = url.split('?');
        let file = parts.next().ok_or("Missing file path")?.to_string();
        let mut line = None;
        let mut column = None;
        if let Some(query) = parts.next() {
            for param in query.split('&') {
                let mut kv = param.split('=');
                match (kv.next(), kv.next()) {
                    (Some("line"), Some(l)) => line = l.parse().ok(),
                    (Some("column"), Some(c)) => column = c.parse().ok(),
                    _ => return Err(format!("Invalid query parameter: {}", param)),
                }
            }
        }

        Ok(OpenPosition { file, line, column })
    }
}