use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);

#[test]
fn run_records_stdout_stderr_and_nonzero_exit() {
    let temp = TempDir::new();

    let run_dir = coldtrace(&temp)
        .args([
            "run",
            "--name",
            "exit-seven",
            "--",
            "sh",
            "-c",
            "printf 'out'; printf 'err' >&2; exit 7",
        ])
        .assert_success_and_run_dir(temp.path());

    assert_eq!(
        fs::read_to_string(run_dir.join("stdout.log")).unwrap(),
        "out"
    );
    assert_eq!(
        fs::read_to_string(run_dir.join("stderr.log")).unwrap(),
        "err"
    );

    let record = read_record(&run_dir);
    assert_eq!(record["schema_version"], "coldtrace.run.v0");
    assert_eq!(
        record["run_id"].as_str(),
        Some(run_dir.file_name().unwrap().to_string_lossy().as_ref())
    );
    assert!(
        run_dir
            .file_name()
            .unwrap()
            .to_string_lossy()
            .ends_with("-exit-seven"),
        "run id should end with the explicit run name"
    );
    assert_eq!(record["name"], "exit-seven");
    assert_eq!(record["command"]["argv"][0], "sh");
    assert_eq!(record["command"]["argv"][1], "-c");
    assert_eq!(
        record["command"]["argv"][2],
        "printf 'out'; printf 'err' >&2; exit 7"
    );
    assert_eq!(record["command"]["cwd"], temp.path().display().to_string());
    assert_eq!(record["exit"]["code"], 7);
    assert_eq!(record["exit"]["signal"], Value::Null);
    assert_eq!(record["output"]["stdout_bytes"], 3);
    assert_eq!(record["output"]["stderr_bytes"], 3);
    assert_eq!(record["artifacts"]["stdout"], "stdout.log");
    assert_eq!(record["artifacts"]["stderr"], "stderr.log");
}

#[test]
#[cfg(unix)]
fn run_records_signal_when_child_is_killed() {
    let temp = TempDir::new();

    let run_dir = coldtrace(&temp)
        .args([
            "run",
            "--name",
            "sigterm",
            "--",
            "sh",
            "-c",
            "kill -TERM $$",
        ])
        .assert_success_and_run_dir(temp.path());

    let record = read_record(&run_dir);
    assert_eq!(record["name"], "sigterm");
    assert_eq!(record["exit"]["code"], Value::Null);
    assert_eq!(record["exit"]["signal"], 15);
}

fn coldtrace(temp: &TempDir) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_coldtrace"));
    command.current_dir(temp.path());
    command
}

fn read_record(run_dir: &Path) -> Value {
    let record = fs::read_to_string(run_dir.join("record.json")).unwrap();
    serde_json::from_str(&record).unwrap()
}

trait CommandAssert {
    fn assert_success_and_run_dir(&mut self, cwd: &Path) -> PathBuf;
}

impl CommandAssert for Command {
    fn assert_success_and_run_dir(&mut self, cwd: &Path) -> PathBuf {
        let output = self.output().unwrap();

        assert!(
            output.status.success(),
            "coldtrace failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );

        let stdout = String::from_utf8(output.stdout).unwrap();
        let run_dir = stdout.trim();
        assert!(
            !run_dir.is_empty(),
            "coldtrace did not print a run directory"
        );

        cwd.join(run_dir)
    }
}

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new() -> Self {
        let id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("coldtrace-test-{}-{now}-{id}", std::process::id()));

        fs::create_dir_all(&path).unwrap();
        let path = fs::canonicalize(path).unwrap();

        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
