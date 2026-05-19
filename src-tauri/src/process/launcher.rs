use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;

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

    fn launch_process(wechat_path: &str) -> Result<u32, String> {
        let alive_pids = manager::get_all_wechat_pids();
        log::info!("[launcher] 当前存活的微信进程PID: {:?}", alive_pids);

        if !alive_pids.is_empty() {
            log::info!("[launcher] 检测到微信进程运行中，开始关闭Mutex...");
            let closed = manager::close_all_wechat_mutexes()?;
            log::info!("[launcher] 关闭了 {} 个Mutex句柄", closed);
            thread::sleep(Duration::from_millis(500));
        }

        log::info!("[launcher] 启动命令: {}", wechat_path);

        let child = Command::new(wechat_path)
            .spawn()
            .map_err(|e| format!("启动微信失败: {}", e))?;

        let pid = child.id();
        log::info!("[launcher] 进程已启动, PID={}", pid);

        thread::sleep(Duration::from_millis(1500));

        let alive_after = manager::get_all_wechat_pids();
        if alive_after.contains(&pid) {
            log::info!("[launcher] 进程 PID={} 1.5秒后仍然存活 ✓", pid);
        } else {
            log::warn!("[launcher] 进程 PID={} 1.5秒后已退出！", pid);
        }

        Ok(pid)
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

        let pid = Self::launch_process(&wechat_path)?;
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
        let pid = Self::launch_process(&wechat_path)?;
        let hwnd = format!("0x{:X}", pid);

        self.db.update_instance_running(instance_id, pid, &hwnd)?;

        log::info!("[launcher] 实例重新启动成功: id={}, pid={}", instance_id, pid);
        Ok(LaunchInfo { instance_id: instance_id.to_string(), pid, hwnd })
    }
}
