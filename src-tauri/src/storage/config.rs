use crate::storage::database::Database;

const DEFAULT_CONFIGS: &[(&str, &str)] = &[
    ("max_instances", "5"),
    ("auto_detect_path", "true"),
    ("minimize_to_tray", "false"),
    ("theme", "light"),
    ("auto_start", "false"),
];

pub fn init_default_configs(db: &Database) -> Result<(), String> {
    for (key, value) in DEFAULT_CONFIGS {
        if db.get_config(key)?.is_none() {
            db.set_config(key, value)?;
        }
    }
    Ok(())
}