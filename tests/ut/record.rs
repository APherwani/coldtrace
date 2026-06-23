use coldtrace::record::{DirectRunRecord, RunRecord};
use serde_json::Value;

#[test]
fn direct_record_serializes_the_c0_schema_shape() {
    let record = RunRecord::direct(DirectRunRecord {
        run_id: "1234-99-torch-import".to_string(),
        name: Some("torch-import".to_string()),
        command_argv: vec![
            "python3".to_string(),
            "-c".to_string(),
            "import torch".to_string(),
        ],
        cwd: "/work/coldtrace".to_string(),
        started_unix_ms: 1234,
        wall_ms: 567,
        exit_code: Some(0),
        exit_signal: None,
        pid: 99,
        stdout_bytes: 12,
        stderr_bytes: 0,
    });

    let json = serde_json::to_value(record).unwrap();

    assert_eq!(json["schema_version"], "coldtrace.run.v0");
    assert_eq!(json["run_id"], "1234-99-torch-import");
    assert_eq!(json["name"], "torch-import");
    assert_eq!(json["launcher"]["mode"], "direct");
    assert_eq!(json["command"]["argv"][2], "import torch");
    assert_eq!(json["timing"]["wall_ms"], 567);
    assert_eq!(json["exit"]["code"], 0);
    assert_eq!(json["exit"]["signal"], Value::Null);
    assert_eq!(json["process"]["pid"], 99);
    assert_eq!(json["output"]["stdout_bytes"], 12);
    assert_eq!(json["artifacts"]["stdout"], "stdout.log");
    assert_eq!(json["artifacts"]["proc_maps"], Value::Null);
}
