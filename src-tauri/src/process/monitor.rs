use crate::mutex::manager;
use crate::storage::database::Database;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

pub fn start_process_monitor(app_handle: AppHandle) {
    std::thread::spawn(move || {
        let mut last_known_pids: Vec<u32> = Vec::new();
        loop {
            std::thread::sleep(Duration::from_secs(2));
            let current_pids = manager::get_all_wechat_pids();

            if !last_known_pids.is_empty() {
                let exited: Vec<u32> = last_known_pids
                    .iter()
                    .filter(|pid| !current_pids.contains(pid))
                    .copied()
                    .collect();

                if !exited.is_empty() {
                    if let Ok(db) = Database::new() {
                        if let Ok(instances) = db.get_all_instances() {
                            for inst in &instances {
                                if inst.status == "running" {
                                    let pid = inst.pid as u32;
                                    if pid > 0 && exited.contains(&pid) {
                                        let _ = db.terminate_instance(&inst.id);
                                    }
                                }
                            }
                        }
                    }
                    let _ = app_handle.emit("wechat-process-exited", ());
                }
            }

            last_known_pids = current_pids;
        }
    });
}
