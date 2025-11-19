use std::env;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

pub fn register() {
    let key = Path::new("Software").join("Classes").join("openwith");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (openwith_key, _) = hkcu.create_subkey(&key).unwrap();
    openwith_key.set_value("URL Protocol", &"").unwrap();
    openwith_key.set_value("", &"URL:openwith").unwrap();
    let (subkey, _) = openwith_key.create_subkey(r"shell\open\command").unwrap();
    // Use the path of this program as the executable
    let exe_path = env::current_exe().unwrap();
    let exe_path = exe_path.to_str().unwrap();
    subkey
        .set_value("", &format!("\"{}\" --open \"%1\"", exe_path))
        .unwrap();
}

pub fn unregister() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = Path::new("Software").join("Classes").join("openwith");
    hkcu.delete_subkey_all(&key).unwrap();
}
