use std::env;
use std::path::Path;
use winreg::enums::*;
use windows::{
  core::*,
  Win32::UI::WindowsAndMessaging::*
};
use winreg::RegKey;

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
            let message = format!("Opening URL: {}", url);
            // Here you would add the code to open the URL with your application
            // For demonstration, we will just print the URL
            // and show a message box
            // Convert the URL to a wide string
            let wide_message: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();
            unsafe {
                MessageBoxW(
                    None,
                    PCWSTR(wide_message.as_ptr()),
                    w!("Open with"),
                    MB_OK,
                );
            }
            // Here you would add the code to open the URL with your application
        } else {
            let message = format!("Unknown argument: {}", arg);
            // Convert the message to a wide string
            let wide_message: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();
            unsafe {
                MessageBoxW(
                    None,
                    PCWSTR(wide_message.as_ptr()),
                    w!("Error"),
                    MB_OK,
                );
            }
        }
    }

    Ok(())
}
