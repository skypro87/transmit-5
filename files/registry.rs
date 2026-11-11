//! Windows registry read helpers for Transmit 5.

pub const APP_KEY: &str = "Software\\Transmit\\Transmit 5";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Transmit 5";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Transmit 5") },
        RegValue { name: "Version",     value: String::from("v1.2.2") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Transmit 5") },
        RegValue { name: "Publisher",       value: String::from("Transmit") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Transmit 5\\uninstall.exe\"") },
    ]
}
