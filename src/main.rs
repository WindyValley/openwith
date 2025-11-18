use std::env;
use std::path::Path;
use winreg::enums::*;
use windows::{
  core::*,
  Win32::UI::WindowsAndMessaging::*
};
use winreg::RegKey;
mod config;
use crate::config::datadef::{Config, OpenPosition};

fn main() -> Result<()> {
    let key = Path::new("Software")
        .join("Classes")
        .join("openwith");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (openwith_key, _) = hkcu.create_subkey(&key).unwrap();
    let mut args = env::args();
    args.next(); // Skip the first argument (the program name)
    while let Some(arg) = args.next() {
        if arg == "--register" {
            openwith_key.set_value("URL Protocol", &"").unwrap();
            openwith_key.set_value("", &"URL:openwith").unwrap();
            let (subkey, _) = openwith_key.create_subkey(r"shell\open\command").unwrap();
            // Use the path of this program as the executable
            let exe_path = env::current_exe().unwrap();
            let exe_path = exe_path.to_str().unwrap();
            subkey.set_value("", &format!("\"{}\" --open \"%1\"", exe_path)).unwrap();
        } else if arg == "--unregister" {
            hkcu.delete_subkey_all(&key).unwrap();
        } else if arg == "--open" {
            let url = args.next().unwrap();

            // show_message(&url, "Open With");

            // Here you would add the code to open the URL with your application
            let open_position = OpenPosition::parse(&url).unwrap();
            let home_dir = dirs::home_dir().expect("Cannot determine home directory");
            let config_path = home_dir.join(".config").join("openwith").join("config.json");
            let config_path = config_path.to_str().unwrap();
            // Assuming you have a way to get the appropriate OpenHandler
            let config = Config::from_json_file(config_path).unwrap();
            if let Some(handler) = config.open_handlers.iter().find(|h| h.id == config.default_open_handler.as_deref().unwrap_or("") || config.default_open_handler.is_none()) {
                handler.do_open(&open_position);
            } else {
                show_message("No valid open handler found.", "Error");
            }
        } else {
            let message = format!("Unknown argument: {}", arg);
            show_message(&message, "Error");
        }
    }

    Ok(())
}

fn show_message(message: &str, title: &str) {
    let wide_message: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_title: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        MessageBoxW(
            None,
            PCWSTR(wide_message.as_ptr()),
            PCWSTR(wide_title.as_ptr()),
            MB_OK,
        );
    }
}