use rusqlite::{Connection, params};
use crate::storage::models::*;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self, String> {
        let db_path = dirs::data_dir()
            .ok_or("无法获取数据目录")?
            .join("械式微信多开器")
            .join("app.db");

        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("创建数据库目录失败: {}", e))?;
        }

        let conn = Connection::open(&db_path)
            .map_err(|e| format!("打开数据库失败: {}", e))?;

        conn.execute_batch("PRAGMA journal_mode=WAL;")
            .map_err(|e| format!("设置WAL模式失败: {}", e))?;

        let db = Database { conn };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<(), String> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS instances (
                id          TEXT PRIMARY KEY,
                label       TEXT,
                pid         INTEGER DEFAULT 0,
                hwnd        TEXT,
                data_path   TEXT,
                status      TEXT DEFAULT 'stopped',
                created_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at  DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS app_config (
                key         TEXT PRIMARY KEY,
                value       TEXT,
                updated_at  DATETIME DEFAULT CURRENT_TIMESTAMP
            );"
        ).map_err(|e| format!("初始化数据库失败: {}", e))?;

        self.run_migrations()?;
        Ok(())
    }

    fn run_migrations(&self) -> Result<(), String> {
        let columns: Vec<String> = self.conn
            .prepare("PRAGMA table_info(instances)")
            .map_err(|e| format!("查询表结构失败: {}", e))?
            .query_map([], |row| row.get::<_, String>(1))
            .map_err(|e| format!("读取列信息失败: {}", e))?
            .filter_map(|r| r.ok())
            .collect();

        if !columns.iter().any(|c| c == "updated_at") {
            self.conn.execute_batch(
                "ALTER TABLE instances ADD COLUMN updated_at DATETIME DEFAULT '1970-01-01 00:00:00';"
            ).map_err(|e| format!("迁移 instances.updated_at 失败: {}", e))?;
        }

        if !columns.iter().any(|c| c == "created_at") {
            self.conn.execute_batch(
                "ALTER TABLE instances ADD COLUMN created_at DATETIME DEFAULT '1970-01-01 00:00:00';"
            ).map_err(|e| format!("迁移 instances.created_at 失败: {}", e))?;
        }

        let config_columns: Vec<String> = self.conn
            .prepare("PRAGMA table_info(app_config)")
            .map_err(|e| format!("查询表结构失败: {}", e))?
            .query_map([], |row| row.get::<_, String>(1))
            .map_err(|e| format!("读取列信息失败: {}", e))?
            .filter_map(|r| r.ok())
            .collect();

        if !config_columns.iter().any(|c| c == "updated_at") {
            self.conn.execute_batch(
                "ALTER TABLE app_config ADD COLUMN updated_at DATETIME DEFAULT '1970-01-01 00:00:00';"
            ).map_err(|e| format!("迁移 app_config.updated_at 失败: {}", e))?;
        }

        Ok(())
    }

    pub fn insert_instance_full(
        &self, id: &str, label: &str, pid: u32, hwnd: &str, data_path: &str,
    ) -> Result<(), String> {
        self.conn.execute(
            "INSERT INTO instances (id, label, pid, hwnd, data_path, status)
             VALUES (?1, ?2, ?3, ?4, ?5, 'running')",
            params![id, label, pid, hwnd, data_path],
        ).map_err(|e| format!("插入实例失败: {}", e))?;
        Ok(())
    }

    pub fn get_all_instances(&self) -> Result<Vec<Instance>, String> {
        let mut stmt = self.conn.prepare(
            "SELECT id, label, pid, hwnd, data_path, status, created_at, updated_at
             FROM instances ORDER BY created_at DESC"
        ).map_err(|e| format!("准备查询失败: {}", e))?;

        let instances = stmt.query_map([], |row| {
            Ok(Instance {
                id: row.get(0)?,
                label: row.get(1)?,
                pid: row.get(2)?,
                hwnd: row.get(3)?,
                data_path: row.get(4)?,
                status: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        }).map_err(|e| format!("查询实例失败: {}", e))?;

        instances.collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("收集实例数据失败: {}", e))
    }

    pub fn update_instance_label(&self, id: &str, label: &str) -> Result<(), String> {
        self.conn.execute(
            "UPDATE instances SET label = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            params![label, id],
        ).map_err(|e| format!("更新标签失败: {}", e))?;
        Ok(())
    }

    pub fn terminate_instance(&self, id: &str) -> Result<(), String> {
        self.conn.execute(
            "UPDATE instances SET status = 'stopped', pid = 0, hwnd = '',
             updated_at = CURRENT_TIMESTAMP WHERE id = ?1",
            params![id],
        ).map_err(|e| format!("更新实例状态失败: {}", e))?;
        Ok(())
    }

    pub fn get_instance(&self, id: &str) -> Result<Option<Instance>, String> {
        let mut stmt = self.conn.prepare(
            "SELECT id, label, pid, hwnd, data_path, status, created_at, updated_at
             FROM instances WHERE id = ?1"
        ).map_err(|e| format!("准备查询失败: {}", e))?;

        let result = stmt.query_row(params![id], |row| {
            Ok(Instance {
                id: row.get(0)?,
                label: row.get(1)?,
                pid: row.get(2)?,
                hwnd: row.get(3)?,
                data_path: row.get(4)?,
                status: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        });
        match result {
            Ok(inst) => Ok(Some(inst)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("查询实例失败: {}", e)),
        }
    }

    pub fn update_instance_running(&self, id: &str, pid: u32, hwnd: &str) -> Result<(), String> {
        self.conn.execute(
            "UPDATE instances SET status = 'running', pid = ?1, hwnd = ?2,
             updated_at = CURRENT_TIMESTAMP WHERE id = ?3",
            params![pid, hwnd, id],
        ).map_err(|e| format!("更新实例运行状态失败: {}", e))?;
        Ok(())
    }

    pub fn delete_instance(&self, id: &str) -> Result<(), String> {
        self.conn.execute(
            "DELETE FROM instances WHERE id = ?1",
            params![id],
        ).map_err(|e| format!("删除实例失败: {}", e))?;
        Ok(())
    }

    pub fn delete_stopped_instances(&self) -> Result<usize, String> {
        let affected = self.conn.execute(
            "DELETE FROM instances WHERE status = 'stopped'",
            [],
        ).map_err(|e| format!("删除已停止实例失败: {}", e))?;
        Ok(affected)
    }

    pub fn get_config(&self, key: &str) -> Result<Option<String>, String> {
        let mut stmt = self.conn.prepare(
            "SELECT value FROM app_config WHERE key = ?1"
        ).map_err(|e| format!("准备查询配置失败: {}", e))?;

        let result = stmt.query_row(params![key], |row| row.get::<_, String>(0));
        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("查询配置失败: {}", e)),
        }
    }

    pub fn set_config(&self, key: &str, value: &str) -> Result<(), String> {
        self.conn.execute(
            "INSERT INTO app_config (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = CURRENT_TIMESTAMP",
            params![key, value],
        ).map_err(|e| format!("保存配置失败: {}", e))?;
        Ok(())
    }

    pub fn get_all_config(&self) -> Result<std::collections::HashMap<String, String>, String> {
        let mut stmt = self.conn.prepare(
            "SELECT key, value FROM app_config"
        ).map_err(|e| format!("准备查询配置失败: {}", e))?;

        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }).map_err(|e| format!("查询配置失败: {}", e))?;

        let mut map = std::collections::HashMap::new();
        for row in rows {
            let (key, value) = row.map_err(|e| format!("读取配置行失败: {}", e))?;
            map.insert(key, value);
        }
        Ok(map)
    }
}
