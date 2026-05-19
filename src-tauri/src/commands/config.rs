use std::collections::HashMap;
use crate::storage::database::Database;

#[tauri::command]
pub async fn get_app_config() -> Result<HashMap<String, String>, String> {
    let db = Database::new()?;
    db.get_all_config()
}

#[tauri::command]
pub async fn save_app_config(key: String, value: String) -> Result<(), String> {
    let db = Database::new()?;
    db.set_config(&key, &value)
}

#[tauri::command]
pub async fn set_minimize_to_tray(enabled: bool) -> Result<(), String> {
    let db = Database::new()?;
    db.set_config("minimize_to_tray", if enabled { "true" } else { "false" })?;
    crate::set_minimize_to_tray(enabled);
    Ok(())
}