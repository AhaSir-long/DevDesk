use devdesk_lib::model::{InstanceKind, OperationRecord, ServiceInstance, ServiceInstanceDraft};
use devdesk_lib::store::open;
use uuid::Uuid;

fn windows_service(name: &str, service_name: &str) -> ServiceInstance {
    ServiceInstance {
        id: Uuid::new_v4(),
        name: name.to_owned(),
        kind: InstanceKind::WindowsService,
        service_name: Some(service_name.to_owned()),
        start_command: None,
        work_directory: None,
        stop_command: None,
        expected_port: Some(3306),
        config_path: None,
        log_path: None,
    }
}

#[test]
fn same_display_name_keeps_distinct_service_instances_after_reopen() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("devdesk.sqlite");
    let first = windows_service("MySQL", "MySQL80");
    let second = windows_service("MySQL", "MySQL57");

    {
        let store = open(&path).unwrap();
        store.save_instance(&first).unwrap();
        store.save_instance(&second).unwrap();
    }

    let saved = open(&path).unwrap().list_instances().unwrap();
    assert_eq!(saved.len(), 2);
    assert_eq!(saved[0].id, first.id);
    assert_eq!(saved[1].id, second.id);
    assert_eq!(saved[0].service_name.as_deref(), Some("MySQL80"));
    assert_eq!(saved[1].service_name.as_deref(), Some("MySQL57"));
}

#[test]
fn operation_history_keeps_codes_without_sensitive_fields() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("devdesk.sqlite");
    let instance = windows_service("Redis", "Redis");
    let record = OperationRecord {
        id: Uuid::new_v4(),
        instance_id: instance.id,
        action: "start".to_owned(),
        outcome: "failed".to_owned(),
        error_code: Some("ACCESS_DENIED".to_owned()),
        occurred_at: 1_700_000_000,
    };

    {
        let store = open(&path).unwrap();
        store.save_instance(&instance).unwrap();
        store.append_operation(&record).unwrap();
    }

    let saved = open(&path).unwrap().list_operations(instance.id).unwrap();
    assert_eq!(saved, vec![record]);
    let raw = std::fs::read(&path).unwrap();
    assert!(
        !raw.windows(b"password".len())
            .any(|value| value == b"password")
    );
    assert!(
        !raw.windows(b"environment".len())
            .any(|value| value == b"environment")
    );
}

#[test]
fn invalid_windows_service_is_not_written() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("devdesk.sqlite");
    let mut instance = windows_service("", "MySQL80");
    let store = open(&path).unwrap();

    assert!(store.save_instance(&instance).is_err());
    instance.name = "MySQL".to_owned();
    instance.service_name = None;
    assert!(store.save_instance(&instance).is_err());
    instance.service_name = Some("MySQL80".to_owned());
    instance.expected_port = Some(0);
    assert!(store.save_instance(&instance).is_err());
    assert!(store.list_instances().unwrap().is_empty());
}

#[test]
fn operation_history_rejects_free_form_error_text() {
    let temp = tempfile::tempdir().unwrap();
    let store = open(&temp.path().join("devdesk.sqlite")).unwrap();
    let instance = windows_service("Redis", "Redis");
    store.save_instance(&instance).unwrap();

    let record = OperationRecord {
        id: Uuid::new_v4(),
        instance_id: instance.id,
        action: "start".to_owned(),
        outcome: "failed".to_owned(),
        error_code: Some("password=secret".to_owned()),
        occurred_at: 1_700_000_000,
    };

    assert!(store.append_operation(&record).is_err());
    assert!(store.list_operations(instance.id).unwrap().is_empty());
}

#[test]
fn registration_assigns_id_and_preserves_frontend_fields() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("devdesk.sqlite");
    let draft: ServiceInstanceDraft = serde_json::from_value(serde_json::json!({
        "name": "MySQL",
        "kind": "windowsService",
        "serviceName": "MySQL80",
        "expectedPort": 3306,
        "configPath": "C:/mysql/my.ini",
        "logPath": null
    }))
    .unwrap();

    let store = open(&path).unwrap();
    let id = store.register_instance(draft).unwrap();
    let saved = store.list_instances().unwrap();
    assert_eq!(saved.len(), 1);
    assert_eq!(saved[0].id, id);
    assert_eq!(saved[0].service_name.as_deref(), Some("MySQL80"));
    assert_eq!(saved[0].config_path.as_deref(), Some("C:/mysql/my.ini"));
    let json = serde_json::to_value(&saved[0]).unwrap();
    assert_eq!(json["serviceName"], "MySQL80");
    assert_eq!(json["expectedPort"], 3306);
}
