use crate::config::datadef::{OpenHandler, OpenPosition};

impl OpenHandler {
    pub fn do_open(&self, pos: &OpenPosition) {
        let mut command = std::process::Command::new(&self.program);
        if let Some(env) = &self.env {
            for (key, value) in env {
                command.env(key, value);
            }
        }
        if let Some(cwd) = &self.cwd {
            command.current_dir(cwd);
        }

        // Replace placeholders in args
        let mut processed_args = Vec::new();
        for arg in self.args.iter() {
            let replaced = arg
                .replace("{file}", &pos.file.replace("/", "\\"))
                .replace("{file_with_slash}", &pos.file.replace("\\", "/"))
                .replace("{line}", &pos.line.map_or("1".to_string(), |l| l.to_string()))
                .replace("{column}", &pos.column.map_or("1".to_string(), |c| c.to_string()));
            processed_args.push(replaced);
        }
        command.args(&processed_args);

        // Execute the command and no wait
        match command.spawn() {
            Ok(_) => { }
            Err(e) => eprintln!("Failed to execute command: {}", e),
        }
    }
}