use std::env;
use windows::{
  core::*,
  Win32::UI::WindowsAndMessaging::*
};
mod config;
mod executor;
use crate::config::datadef::OpenPosition;

fn main() -> Result<()> {
    let mut args = env::args();
    args.next(); // Skip the first argument (the program name)
    while let Some(arg) = args.next() {
        if arg == "--register" {
            executor::urlregister::register();  
        } else if arg == "--unregister" {
            executor::urlregister::unregister();
        } else if arg == "--open" {
            let url = args.next().unwrap();

            // show_message(&url, "Open With");

            // Here you would add the code to open the URL with your application
            let open_position = OpenPosition::parse(&url).unwrap();
            let manager = config::manager::Manager::new();
            if let Ok(handler) = manager.get_current_handler() {
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