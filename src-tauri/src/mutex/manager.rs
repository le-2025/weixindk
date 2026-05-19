use std::ffi::c_void;
use std::fs;
use std::path::PathBuf;

use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;

use crate::mutex::names::WECHAT_MUTEX_NAMES;

#[repr(C)]
struct SystemHandleTableEntryInfo {
    unique_process_id: u16,
    creator_back_trace_index: u16,
    object_type_index: u8,
    handle_attributes: u8,
    handle_value: u16,
    object: *mut c_void,
    granted_access: u32,
}

#[repr(C)]
struct SystemHandleInformationEx {
    number_of_handles: u32,
    reserved: u32,
    handles: [SystemHandleTableEntryInfo; 1],
}

extern "system" {
    fn NtQuerySystemInformation(
        system_information_class: u32,
        system_information: *mut c_void,
        system_information_length: u32,
        return_length: *mut u32,
    ) -> i32;
}

const SYSTEM_HANDLE_INFORMATION: u32 = 16;
const STATUS_INFO_LENGTH_MISMATCH: i32 = 0xC0000004u32 as i32;
const STATUS_SUCCESS: i32 = 0;

pub fn get_all_wechat_pids() -> Vec<u32> {
    use sysinfo::{System, ProcessesToUpdate};
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All);

    let mut pids: Vec<u32> = sys
        .processes()
        .iter()
        .filter(|(_, p)| {
            let name = p.name().to_str().unwrap_or("").to_lowercase();
            name == "wechat.exe" || name == "weixin.exe"
        })
        .map(|(pid, _)| pid.as_u32())
        .collect();

    pids.sort_by(|a, b| b.cmp(a));
    pids
}

pub fn close_all_wechat_mutexes() -> Result<u32, String> {
    let pids = get_all_wechat_pids();
    if pids.is_empty() {
        return Ok(0);
    }

    let mut total_closed = 0u32;
    for &pid in &pids {
        match close_mutexes_for_process(pid) {
            Ok(count) => total_closed += count,
            Err(e) => log::warn!("关闭进程 {} 的Mutex失败: {}", pid, e),
        }
    }

    Ok(total_closed)
}

fn close_mutexes_for_process(target_pid: u32) -> Result<u32, String> {
    unsafe {
        let process = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_DUP_HANDLE,
            false,
            target_pid,
        )
        .map_err(|e| format!("无法打开进程 {}: {:?}", target_pid, e))?;

        let mut buffer_size: u32 = 0x100000;
        let mut closed_count = 0u32;

        for _retry in 0..3 {
            let mut buffer: Vec<u8> = vec![0u8; buffer_size as usize];
            let mut return_length: u32 = 0;

            let status = NtQuerySystemInformation(
                SYSTEM_HANDLE_INFORMATION,
                buffer.as_mut_ptr() as *mut c_void,
                buffer_size,
                &mut return_length,
            );

            if status == STATUS_INFO_LENGTH_MISMATCH {
                buffer_size = return_length * 2;
                continue;
            }

            if status != STATUS_SUCCESS {
                CloseHandle(process).ok();
                return Err(format!(
                    "NtQuerySystemInformation 失败: 0x{:X}",
                    status as u32
                ));
            }

            let header = &*(buffer.as_ptr() as *const SystemHandleInformationEx);
            let handle_count = header.number_of_handles as usize;
            let handles_ptr = buffer
                .as_ptr()
                .add(std::mem::size_of::<u32>() * 2)
                as *const SystemHandleTableEntryInfo;
            let handles = std::slice::from_raw_parts(handles_ptr, handle_count);

            for entry in handles.iter() {
                let entry_pid = entry.unique_process_id as u32;
                if entry_pid != target_pid {
                    continue;
                }
                if entry.object_type_index < 20 || entry.object_type_index > 30 {
                    continue;
                }

                let mut dup_handle = HANDLE::default();
                if DuplicateHandle(
                    process,
                    HANDLE(entry.handle_value as usize as *mut c_void),
                    GetCurrentProcess(),
                    &mut dup_handle,
                    0,
                    false,
                    DUPLICATE_SAME_ACCESS,
                )
                .is_err()
                {
                    continue;
                }

                if check_if_wechat_mutex_name(dup_handle) {
                    let mut close_handle = HANDLE::default();
                    if DuplicateHandle(
                        process,
                        HANDLE(entry.handle_value as usize as *mut c_void),
                        GetCurrentProcess(),
                        &mut close_handle,
                        0,
                        false,
                        DUPLICATE_CLOSE_SOURCE | DUPLICATE_SAME_ACCESS,
                    )
                    .is_ok()
                    {
                        CloseHandle(close_handle).ok();
                        closed_count += 1;
                    }
                }

                CloseHandle(dup_handle).ok();
            }

            break;
        }

        CloseHandle(process).ok();
        Ok(closed_count)
    }
}

fn check_if_wechat_mutex_name(_handle: HANDLE) -> bool {
    for name in WECHAT_MUTEX_NAMES.iter() {
        unsafe {
            let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
            let result = OpenMutexW(
                MUTEX_ALL_ACCESS,
                false,
                windows::core::PCWSTR(name_wide.as_ptr()),
            );
            if let Ok(mutex_handle) = result {
                CloseHandle(mutex_handle).ok();
                return true;
            }
        }
    }
    false
}

pub fn prepare_instance_dir(instance_id: &str) -> Result<PathBuf, String> {
    let base_dir = dirs::data_dir()
        .ok_or("无法获取数据目录")?
        .join("械式微信多开器");

    let instance_dir = base_dir.join("instances").join(instance_id);

    fs::create_dir_all(instance_dir.join("Documents").join("WeChat Files"))
        .map_err(|e| format!("创建实例目录失败: {}", e))?;
    fs::create_dir_all(instance_dir.join("AppData").join("Roaming"))
        .map_err(|e| format!("创建AppData目录失败: {}", e))?;
    fs::create_dir_all(instance_dir.join("Temp"))
        .map_err(|e| format!("创建Temp目录失败: {}", e))?;

    Ok(instance_dir)
}