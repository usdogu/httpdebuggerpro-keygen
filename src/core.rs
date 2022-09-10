use rand::{thread_rng, Rng};
use winreg::RegKey;
use winsafe::GetVolumeInformation;

pub fn write_to_registry(
    registry_key_name: &str,
    license_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let (regkey, _) = hkcu.create_subkey("Software\\MadeForNet\\HTTPDebuggerPro")?;
    regkey.set_value(registry_key_name, &license_key.to_string())?;

    Ok(())
}

pub fn get_httpdebugger_version() -> Result<u32, Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let (regkey, _) = hkcu.create_subkey("Software\\MadeForNet\\HTTPDebuggerPro")?;
    let version_full: String = regkey.get_value("AppVer")?;
    let mut version = version_full
        .split_whitespace()
        .last()
        .expect("There's an error while reading the Http Debugger version")
        .to_string();
    version.retain(|c| c != '.'); // remove dots from the version number

    Ok(version.parse()?)
}

pub fn create_registry_key_name(
    httpdebugger_version: u32,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut serial_number = u32::default();

    GetVolumeInformation(
        Some("C:\\"),
        None,
        Some(&mut serial_number),
        None,
        None,
        None,
    )?;
    let result = format!(
        "SN{}",
        httpdebugger_version ^ ((!serial_number >> 1) + 736) ^ 0x590D4
    );

    Ok(result)
}

pub fn create_license_key() -> String {
    let mut rng = thread_rng();
    let rand1: u8 = rng.gen();
    let rand2: u8 = rng.gen();
    let rand3: u8 = rng.gen();
    format!(
        "{:02X}{:02X}{:02X}7C{:02X}{:02X}{:02X}{:02X}",
        rand1,
        rand2 ^ 0x7c,
        !rand1,
        rand2,
        rand3,
        rand3 ^ 7,
        rand1 ^ !rand3
    )
}
