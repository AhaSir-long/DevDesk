use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum InstanceKind {
    WindowsService,
    ManualProgram,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceInstance {
    pub id: Uuid,
    pub name: String,
    pub kind: InstanceKind,
    pub service_name: Option<String>,
    pub start_command: Option<String>,
    pub work_directory: Option<String>,
    pub stop_command: Option<String>,
    pub expected_port: Option<u16>,
    pub config_path: Option<String>,
    pub log_path: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceInstanceDraft {
    pub name: String,
    pub kind: InstanceKind,
    pub service_name: Option<String>,
    pub start_command: Option<String>,
    pub work_directory: Option<String>,
    pub stop_command: Option<String>,
    pub expected_port: Option<u16>,
    pub config_path: Option<String>,
    pub log_path: Option<String>,
}

impl ServiceInstanceDraft {
    pub fn with_id(self, id: Uuid) -> ServiceInstance {
        ServiceInstance {
            id,
            name: self.name,
            kind: self.kind,
            service_name: self.service_name,
            start_command: self.start_command,
            work_directory: self.work_directory,
            stop_command: self.stop_command,
            expected_port: self.expected_port,
            config_path: self.config_path,
            log_path: self.log_path,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationRecord {
    pub id: Uuid,
    pub instance_id: Uuid,
    pub action: String,
    pub outcome: String,
    pub error_code: Option<String>,
    pub occurred_at: i64,
}
