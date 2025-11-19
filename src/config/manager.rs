use crate::config::datadef::{Config, OpenHandler};

pub struct Manager {
    config: Config,
}

impl Manager {
    pub fn new() -> Self {
        let config_path = Self::get_config_path();
        let config = Config::from_json_file(&config_path).expect("Failed to load config");
        Manager { config }
    }

    pub fn get_current_handler(&self) -> Result<&OpenHandler, &str> {
        if let Some(default_id) = &self.config.default_open_handler {
            let handler = self.config
                .open_handlers
                .iter()
                .find(|h| &h.id == default_id)
                .expect("Default open handler not found");
            Ok(handler)
        } else if !self.config.open_handlers.is_empty() {
            Ok(&self.config.open_handlers[0])
        } else {
            Err("No open handlers available")
        }
    }

    fn get_config_path() -> String {
        let home_dir = dirs::home_dir().expect("Cannot determine home directory");
        let mut config_path = home_dir.join(".config").join("openwith").join("config.json");
        if config_path.exists() == false {
            config_path = std::env::current_exe().unwrap().parent().unwrap().join("data").join("config.json");
        }
        config_path.to_str().unwrap().to_string()
    }
}