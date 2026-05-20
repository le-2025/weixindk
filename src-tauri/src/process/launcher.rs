use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::CloseHandle;
#[cfg(target_os = "windows")]
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};

use serde::Serialize;

use crate::mutex::manager;
use crate::storage::database::Database;

#[derive(Debug, Clone, Serialize)]
pub struct LaunchInfo {
    pub instance_id: String,
    pub pid: u32,
    pub hwnd: String,
}

pub struct WechatLauncher {
    db: Database,
}

impl WechatLauncher {
    pub fn new() -> Result<Self, String> {
        let db = Database::new()?;
        Ok(WechatLauncher { db })
    }

    fn get_wechat_path(&self) -> Result<String, String> {
        if let Ok(Some(path)) = self.db.get_config("wechat_path") {
            if !path.is_empty() && Path::new(&path).exists() {
                return Ok(path);
            }
        }
        crate::process::registry::get_wechat_install_path()
    }

    fn get_base_dir() -> Result<PathBuf, String> {
        let base = dirs::data_dir()
            .ok_or("无法获取数据目录")?
            .join("械式微信多开器");
        if base.exists() {
            base.canonicalize().map_err(|e| format!("路径解析失败: {}", e))
        } else {
            Ok(base)
        }
    }

    fn ensure_wechat_data_dirs(data_dir: &Path) -> Result<(), String> {
        let dirs_to_create = [
            "Documents\\WeChat Files",
            "Documents\\xwechat_files",
            "AppData\\Roaming",
            "AppData\\Local",
            "Temp",
        ];
        for dir in &dirs_to_create {
            let full_path = data_dir.join(dir);
            if !full_path.exists() {
                fs::create_dir_all(&full_path)
                    .map_err(|e| format!("创建数据目录 {} 失败: {}", dir, e))?;
            }
        }
        Ok(())
    }

    fn launch_process(wechat_path: &str, data_dir: &Path) -> Result<u32, String> {
        Self::ensure_wechat_data_dirs(data_dir)?;

        let alive_pids = manager::get_all_wechat_pids();
        log::info!("[launcher] 当前存活的微信进程PID: {:?}", alive_pids);

        if !alive_pids.is_empty() {
            log::info!("[launcher] 检测到微信进程运行中，开始关闭Mutex...");
            let closed = manager::close_all_wechat_mutexes()?;
            log::info!("[launcher] 关闭了 {} 个Mutex句柄", closed);
            thread::sleep(Duration::from_millis(800));
        }

        log::info!("[launcher] 启动命令: {} (数据目录: {:?})", wechat_path, data_dir);

        let data_dir_str = data_dir.to_string_lossy().to_string();
        let temp_dir = data_dir.join("Temp").to_string_lossy().to_string();

        let mut cmd = Command::new(wechat_path);
        cmd.env("USERPROFILE", &data_dir_str)
            .env("TEMP", &temp_dir)
            .env("TMP", &temp_dir);

        #[cfg(target_os = "windows")]
        {
            cmd.creation_flags(0x00000010);
        }

        let mut child = cmd
            .spawn()
            .map_err(|e| format!("启动微信失败: {}", e))?;

        let pid = child.id();
        log::info!("[launcher] 进程已启动, PID={}", pid);

        for attempt in 1..=8 {
            thread::sleep(Duration::from_millis(1000));

            match child.try_wait() {
                Ok(Some(status)) => {
                    let exit_code = status.code().unwrap_or(-1);
                    return Err(format!(
                        "微信进程意外退出 (PID={}, exit_code={}, 尝试次数={})。请检查微信路径是否正确。",
                        pid, exit_code, attempt
                    ));
                }
                Ok(None) => {}
                Err(e) => {
                    log::warn!("[launcher] 检查进程状态时出错: {}", e);
                }
            }

            let process_alive = Self::check_process_alive(pid);
            let in_sysinfo_list = {
                let alive_after = manager::get_all_wechat_pids();
                alive_after.contains(&pid)
            };

            log::info!(
                "[launcher] 进程检测 (attempt={}): OpenProcess={}, sysinfo={}",
                attempt, process_alive, in_sysinfo_list
            );

            if process_alive || in_sysinfo_list {
                thread::sleep(Duration::from_millis(500));
                if Self::check_process_alive(pid) {
                    log::info!(
                        "[launcher] 进程 PID={} 确认存活 (attempt={}, via={})",
                        pid,
                        attempt,
                        if process_alive { "OpenProcess" } else { "sysinfo" }
                    );
                    return Ok(pid);
                }
            }
        }

        Err(format!(
            "微信进程 PID={} 启动后未能确认存活，请检查微信路径: {}",
            pid, wechat_path
        ))
    }

    #[cfg(target_os = "windows")]
    fn check_process_alive(pid: u32) -> bool {
        unsafe {
            match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
                Ok(handle) => {
                    CloseHandle(handle).ok();
                    true
                }
                Err(_) => false,
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    fn check_process_alive(_pid: u32) -> bool {
        false
    }

    pub fn launch_new_instance(&self, label: Option<&str>) -> Result<LaunchInfo, String> {
        log::info!("========== launch_new_instance 开始 ==========");

        let max_instances: usize = self.db.get_config("max_instances")
            .ok()
            .flatten()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);

        let alive_pids = manager::get_all_wechat_pids();
        if alive_pids.len() >= max_instances {
            return Err(format!(
                "已达到最大实例数限制 ({}/{})，请先关闭其他微信再试",
                alive_pids.len(), max_instances
            ));
        }

        let wechat_path = self.get_wechat_path()?;
        let instance_id = uuid::Uuid::new_v4().to_string();

        let base_dir = Self::get_base_dir()?;
        let instance_dir = base_dir.join("instances").join(&instance_id);
        fs::create_dir_all(&instance_dir)
            .map_err(|e| format!("创建实例目录失败: {}", e))?;

        let pid = Self::launch_process(&wechat_path, &instance_dir)?;
        let hwnd = format!("0x{:X}", pid);

        self.db.insert_instance_full(
            &instance_id,
            label.unwrap_or(&format!("实例 {}", &instance_id[..8])),
            pid,
            &hwnd,
            &instance_dir.to_string_lossy(),
        )?;

        log::info!("[launcher] 实例创建成功: id={}, pid={}", instance_id, pid);
        Ok(LaunchInfo { instance_id, pid, hwnd })
    }

    pub fn relaunch_instance(&self, instance_id: &str) -> Result<LaunchInfo, String> {
        log::info!("========== relaunch_instance 开始: {} ==========", instance_id);

        let inst = self.db.get_instance(instance_id)?
            .ok_or_else(|| format!("实例 {} 不存在", instance_id))?;

        if inst.status == "running" {
            return Err("该实例已在运行中".into());
        }

        let wechat_path = self.get_wechat_path()?;
        let data_dir = PathBuf::from(&inst.data_path);

        let pid = Self::launch_process(&wechat_path, &data_dir)?;
        let hwnd = format!("0x{:X}", pid);

        self.db.update_instance_running(instance_id, pid, &hwnd)?;

        log::info!("[launcher] 实例重新启动成功: id={}, pid={}", instance_id, pid);
        Ok(LaunchInfo { instance_id: instance_id.to_string(), pid, hwnd })
    }
}
