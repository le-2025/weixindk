use crate::process::launcher::{LaunchInfo, WechatLauncher};
use crate::mutex::manager;
use crate::storage::database::Database;
use crate::storage::models::Instance;

#[tauri::command]
pub async fn launch_wechat(label: Option<String>) -> Result<LaunchInfo, String> {
    let launcher = WechatLauncher::new()?;
    launcher.launch_new_instance(label.as_deref())
}

#[tauri::command]
pub async fn get_wechat_instances() -> Result<Vec<Instance>, String> {
    let db = Database::new()?;
    db.get_all_instances()
}

#[tauri::command]
pub async fn update_instance_label(instance_id: String, label: String) -> Result<(), String> {
    let db = Database::new()?;
    db.update_instance_label(&instance_id, &label)
}

#[tauri::command]
pub async fn terminate_instance(instance_id: String) -> Result<(), String> {
    let db = Database::new()?;
    db.terminate_instance(&instance_id)
}

#[tauri::command]
pub async fn get_wechat_path() -> Result<String, String> {
    crate::process::registry::get_wechat_install_path()
}

#[tauri::command]
pub async fn sync_instances() -> Result<Vec<Instance>, String> {
    let db = Database::new()?;
    let alive_pids = manager::get_all_wechat_pids();
    let instances = db.get_all_instances()?;

    for inst in &instances {
        if inst.status == "running" {
            let pid = inst.pid as u32;
            if pid > 0 && !alive_pids.contains(&pid) {
                db.terminate_instance(&inst.id)?;
            }
        }
    }

    db.get_all_instances()
}