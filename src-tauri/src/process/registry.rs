use windows::Win32::System::Registry::*;
use windows::core::w;
use windows::Win32::Foundation::WIN32_ERROR;

const REGISTRY_PATHS: &[&str] = &[
    "Software\\Tencent\\Weixin",
    "Software\\Tencent\\WeChat",
];

const EXECUTABLE_NAMES: &[&str] = &[
    "Weixin.exe",
    "WeChat.exe",
];

pub fn get_wechat_install_path() -> Result<String, String> {
    for reg_path in REGISTRY_PATHS {
        if let Some(path) = read_install_path_from_registry(reg_path)? {
            for exe_name in EXECUTABLE_NAMES {
                let full_path = format!("{}\\{}", path, exe_name);
                if std::path::Path::new(&full_path).exists() {
                    return Ok(full_path);
                }
            }
        }
    }

    for exe_name in EXECUTABLE_NAMES {
        for default_dir in &[
            r"C:\Program Files\Tencent\Weixin",
            r"C:\Program Files\Tencent\WeChat",
            r"C:\Program Files (x86)\Tencent\Weixin",
            r"C:\Program Files (x86)\Tencent\WeChat",
        ] {
            let full_path = format!("{}\\{}", default_dir, exe_name);
            if std::path::Path::new(&full_path).exists() {
                return Ok(full_path);
            }
        }
    }

    Err("未找到微信安装路径，请在设置中手动指定".into())
}

fn read_install_path_from_registry(reg_path: &str) -> Result<Option<String>, String> {
    unsafe {
        let mut key = windows::Win32::System::Registry::HKEY::default();
        let path_wide: Vec<u16> = reg_path.encode_utf16().chain(std::iter::once(0)).collect();

        let result = RegOpenKeyExW(
            windows::Win32::System::Registry::HKEY_CURRENT_USER,
            windows::core::PCWSTR(path_wide.as_ptr()),
            0,
            KEY_READ,
            &mut key,
        );
        if result != WIN32_ERROR(0) {
            return Ok(None);
        }

        let mut data_type = REG_VALUE_TYPE::default();
        let mut data_size = 512u32;
        let mut buffer = vec![0u16; 256];

        let result = RegQueryValueExW(
            key,
            w!("InstallPath"),
            None,
            Some(&mut data_type),
            Some(buffer.as_mut_ptr() as *mut u8),
            Some(&mut data_size),
        );
        let _ = RegCloseKey(key);

        if result != WIN32_ERROR(0) {
            return Ok(None);
        }

        let len = (data_size as usize / 2).saturating_sub(1);
        let path = String::from_utf16_lossy(&buffer[..len.min(buffer.len())]);
        Ok(Some(path.trim_end_matches('\0').to_string()))
    }
}