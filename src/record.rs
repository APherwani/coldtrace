use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RunRecord {
    pub schema_version: &'static str,
    pub run_id: String,
    pub name: Option<String>,
    pub launcher: LauncherRecord,
    pub command: CommandRecord,
    pub host: HostRecord,
    pub timing: TimingRecord,
    pub exit: ExitRecord,
    pub rusage: RusageRecord,
    pub process: ProcessRecord,
    pub output: OutputRecord,
    pub artifacts: ArtifactsRecord,
}

#[derive(Debug)]
pub struct DirectRunRecord {
    pub run_id: String,
    pub name: Option<String>,
    pub command_argv: Vec<String>,
    pub cwd: String,
    pub started_unix_ms: u128,
    pub wall_ms: u128,
    pub exit_code: Option<i32>,
    pub exit_signal: Option<i32>,
    pub pid: u32,
    pub stdout_bytes: u64,
    pub stderr_bytes: u64,
}

impl RunRecord {
    pub fn direct(input: DirectRunRecord) -> Self {
        Self {
            schema_version: "coldtrace.run.v0",
            run_id: input.run_id,
            name: input.name,
            launcher: LauncherRecord {
                mode: "direct",
                unit_name: None,
            },
            command: CommandRecord {
                argv: input.command_argv,
                cwd: input.cwd,
            },
            host: HostRecord::default(),
            timing: TimingRecord {
                started_unix_ms: input.started_unix_ms,
                wall_ms: input.wall_ms,
                time_to_first_stdout_ms: None,
                time_to_ready_ms: None,
            },
            exit: ExitRecord {
                code: input.exit_code,
                signal: input.exit_signal,
            },
            rusage: RusageRecord::default(),
            process: ProcessRecord {
                pid: input.pid,
                cgroup: None,
                peak_rss_kb: None,
                mapped_files_count: None,
            },
            output: OutputRecord {
                stdout_bytes: input.stdout_bytes,
                stderr_bytes: input.stderr_bytes,
            },
            artifacts: ArtifactsRecord {
                stdout: "stdout.log",
                stderr: "stderr.log",
                proc_status: None,
                proc_cgroup: None,
                proc_maps: None,
                importtime: None,
                strace_summary: None,
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LauncherRecord {
    pub mode: &'static str,
    pub unit_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CommandRecord {
    pub argv: Vec<String>,
    pub cwd: String,
}

#[derive(Debug, Default, Serialize)]
pub struct HostRecord {
    pub kernel: Option<String>,
    pub hostname: Option<String>,
    pub systemd_version: Option<String>,
    pub python_version: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TimingRecord {
    pub started_unix_ms: u128,
    pub wall_ms: u128,
    pub time_to_first_stdout_ms: Option<u128>,
    pub time_to_ready_ms: Option<u128>,
}

#[derive(Debug, Serialize)]
pub struct ExitRecord {
    pub code: Option<i32>,
    pub signal: Option<i32>,
}

#[derive(Debug, Default, Serialize)]
pub struct RusageRecord {
    pub user_cpu_ms: Option<u64>,
    pub system_cpu_ms: Option<u64>,
    pub minor_faults: Option<i64>,
    pub major_faults: Option<i64>,
    pub voluntary_context_switches: Option<i64>,
    pub involuntary_context_switches: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ProcessRecord {
    pub pid: u32,
    pub cgroup: Option<String>,
    pub peak_rss_kb: Option<u64>,
    pub mapped_files_count: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct OutputRecord {
    pub stdout_bytes: u64,
    pub stderr_bytes: u64,
}

#[derive(Debug, Serialize)]
pub struct ArtifactsRecord {
    pub stdout: &'static str,
    pub stderr: &'static str,
    pub proc_status: Option<String>,
    pub proc_cgroup: Option<String>,
    pub proc_maps: Option<String>,
    pub importtime: Option<String>,
    pub strace_summary: Option<String>,
}
