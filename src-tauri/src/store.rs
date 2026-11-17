use std::fmt;
use std::path::Path;

use rusqlite::{Connection, params};
use uuid::Uuid;

use crate::model::{InstanceKind, OperationRecord, ServiceInstance, ServiceInstanceDraft};

#[derive(Debug)]
pub enum AppError {
    Database(rusqlite::Error),
    InvalidInput(&'static str),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(error) => write!(f, "数据库操作失败：{error}"),
            Self::InvalidInput(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<rusqlite::Error> for AppError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Database(error)
    }
}

pub struct Store {
    connection: Connection,
}

pub fn open(path: &Path) -> Result<Store, AppError> {
    let connection = Connection::open(path)?;
    connection.execute_batch(
        "PRAGMA foreign_keys = ON;
         CREATE TABLE IF NOT EXISTS instances (
           id TEXT PRIMARY KEY,
           name TEXT NOT NULL,
           kind TEXT NOT NULL,
           service_name TEXT,
           start_command TEXT,
           work_directory TEXT,
           stop_command TEXT,
           expected_port INTEGER,
           config_path TEXT,
           log_path TEXT
         );
         CREATE TABLE IF NOT EXISTS operations (
           id TEXT PRIMARY KEY,
           instance_id TEXT NOT NULL REFERENCES instances(id),
           action TEXT NOT NULL,
           outcome TEXT NOT NULL,
           error_code TEXT,
           occurred_at INTEGER NOT NULL
         );",
    )?;
    Ok(Store { connection })
}

impl Store {
    pub fn register_instance(&self, draft: ServiceInstanceDraft) -> Result<Uuid, AppError> {
        let instance = draft.with_id(Uuid::new_v4());
        self.save_instance(&instance)?;
        Ok(instance.id)
    }

    pub fn save_instance(&self, instance: &ServiceInstance) -> Result<(), AppError> {
        if instance.name.trim().is_empty() {
            return Err(AppError::InvalidInput("请输入实例名称"));
        }
        match instance.kind {
            InstanceKind::WindowsService
                if instance
                    .service_name
                    .as_deref()
                    .is_none_or(|value| value.trim().is_empty()) =>
            {
                return Err(AppError::InvalidInput("请输入 Windows 服务名"));
            }
            InstanceKind::ManualProgram
                if instance
                    .start_command
                    .as_deref()
                    .is_none_or(|value| value.trim().is_empty()) =>
            {
                return Err(AppError::InvalidInput("请输入启动命令"));
            }
            _ => {}
        }
        if instance.expected_port == Some(0) {
            return Err(AppError::InvalidInput("预期端口必须在 1 到 65535 之间"));
        }
        let kind = match instance.kind {
            InstanceKind::WindowsService => "windowsService",
            InstanceKind::ManualProgram => "manualProgram",
        };
        self.connection.execute(
            "INSERT INTO instances (id, name, kind, service_name, start_command, work_directory,
                                    stop_command, expected_port, config_path, log_path)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT(id) DO UPDATE SET
               name=excluded.name, kind=excluded.kind, service_name=excluded.service_name,
               start_command=excluded.start_command, work_directory=excluded.work_directory,
               stop_command=excluded.stop_command, expected_port=excluded.expected_port,
               config_path=excluded.config_path, log_path=excluded.log_path",
            params![
                instance.id.to_string(),
                instance.name,
                kind,
                instance.service_name,
                instance.start_command,
                instance.work_directory,
                instance.stop_command,
                instance.expected_port,
                instance.config_path,
                instance.log_path,
            ],
        )?;
        Ok(())
    }

    pub fn list_instances(&self) -> Result<Vec<ServiceInstance>, AppError> {
        let mut statement = self.connection.prepare(
            "SELECT id, name, kind, service_name, start_command, work_directory, stop_command,
                    expected_port, config_path, log_path FROM instances ORDER BY rowid",
        )?;
        let rows = statement.query_map([], |row| {
            let id: String = row.get(0)?;
            let kind: String = row.get(2)?;
            Ok(ServiceInstance {
                id: Uuid::parse_str(&id).map_err(|_| rusqlite::Error::InvalidQuery)?,
                name: row.get(1)?,
                kind: match kind.as_str() {
                    "windowsService" => InstanceKind::WindowsService,
                    "manualProgram" => InstanceKind::ManualProgram,
                    _ => return Err(rusqlite::Error::InvalidQuery),
                },
                service_name: row.get(3)?,
                start_command: row.get(4)?,
                work_directory: row.get(5)?,
                stop_command: row.get(6)?,
                expected_port: row.get(7)?,
                config_path: row.get(8)?,
                log_path: row.get(9)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(AppError::from)
    }

    pub fn append_operation(&self, record: &OperationRecord) -> Result<(), AppError> {
        if !safe_code(&record.action)
            || !safe_code(&record.outcome)
            || record
                .error_code
                .as_deref()
                .is_some_and(|value| !safe_code(value))
        {
            return Err(AppError::InvalidInput("操作历史只允许记录结果码"));
        }
        self.connection.execute(
            "INSERT INTO operations (id, instance_id, action, outcome, error_code, occurred_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                record.id.to_string(),
                record.instance_id.to_string(),
                record.action,
                record.outcome,
                record.error_code,
                record.occurred_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_operations(&self, instance_id: Uuid) -> Result<Vec<OperationRecord>, AppError> {
        let mut statement = self.connection.prepare(
            "SELECT id, action, outcome, error_code, occurred_at
             FROM operations WHERE instance_id = ?1 ORDER BY occurred_at, rowid",
        )?;
        let rows = statement.query_map([instance_id.to_string()], |row| {
            let id: String = row.get(0)?;
            Ok(OperationRecord {
                id: Uuid::parse_str(&id).map_err(|_| rusqlite::Error::InvalidQuery)?,
                instance_id,
                action: row.get(1)?,
                outcome: row.get(2)?,
                error_code: row.get(3)?,
                occurred_at: row.get(4)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(AppError::from)
    }
}

fn safe_code(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}
