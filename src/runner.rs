use crate::record::{DirectRunRecord, RunRecord};
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use time::{format_description, OffsetDateTime};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct RunOptions {
    pub name: Option<String>,
    pub command_argv: Vec<String>,
}

pub fn run_command(options: RunOptions) -> Result<PathBuf> {
    let cwd = env::current_dir()?;
    let started_unix_ms = unix_ms()?;
    let label = run_label(options.name.as_deref(), &options.command_argv);
    let base_run_id = run_id(started_unix_ms, &label)?;
    let (run_id, run_dir) = create_run_dir(&base_run_id)?;

    let stdout_path = run_dir.join("stdout.log");
    let stderr_path = run_dir.join("stderr.log");
    let stdout_file = create_file(&stdout_path)?;
    let stderr_file = create_file(&stderr_path)?;

    let start = Instant::now();
    let mut child = Command::new(&options.command_argv[0])
        .args(&options.command_argv[1..])
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .spawn()
        .map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("could not spawn {:?}: {error}", options.command_argv[0]),
            )
        })?;

    let pid = child.id();
    let status = child.wait().map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("could not wait for child {pid}: {error}"),
        )
    })?;
    let wall_ms = start.elapsed().as_millis();

    let record = RunRecord::direct(DirectRunRecord {
        run_id,
        name: options.name,
        command_argv: options.command_argv,
        cwd: cwd.display().to_string(),
        started_unix_ms,
        wall_ms,
        exit_code: status.code(),
        exit_signal: exit_signal(&status),
        pid,
        stdout_bytes: file_len(&stdout_path)?,
        stderr_bytes: file_len(&stderr_path)?,
    });

    let record_path = run_dir.join("record.json");
    let json = serde_json::to_string_pretty(&record)?;
    fs::write(&record_path, format!("{json}\n")).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("could not write {}: {error}", record_path.display()),
        )
    })?;

    Ok(run_dir)
}

fn create_file(path: &Path) -> io::Result<File> {
    File::create(path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("could not create {}: {error}", path.display()),
        )
    })
}

fn exit_signal(status: &ExitStatus) -> Option<i32> {
    #[cfg(unix)]
    {
        status.signal()
    }

    #[cfg(not(unix))]
    {
        let _ = status;
        None
    }
}

fn file_len(path: &Path) -> io::Result<u64> {
    fs::metadata(path)
        .map(|metadata| metadata.len())
        .map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("could not stat {}: {error}", path.display()),
            )
        })
}

fn unix_ms() -> Result<u128> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())
}

fn create_run_dir(base_run_id: &str) -> io::Result<(String, PathBuf)> {
    fs::create_dir_all("runs")?;

    for suffix in 0..1000 {
        let run_id = if suffix == 0 {
            base_run_id.to_string()
        } else {
            format!("{base_run_id}-{suffix}")
        };
        let run_dir = Path::new("runs").join(&run_id);

        match fs::create_dir(&run_dir) {
            Ok(()) => return Ok((run_id, run_dir)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(io::Error::new(
                    error.kind(),
                    format!("could not create {}: {error}", run_dir.display()),
                ));
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        format!("could not allocate a unique run directory for {base_run_id}"),
    ))
}

fn run_id(started_unix_ms: u128, label: &str) -> Result<String> {
    let timestamp = utc_run_timestamp(started_unix_ms)?;

    Ok(format!("{timestamp}-{label}"))
}

fn utc_run_timestamp(started_unix_ms: u128) -> Result<String> {
    let seconds = i64::try_from(started_unix_ms / 1000)?;
    let milliseconds = started_unix_ms % 1000;
    let datetime = OffsetDateTime::from_unix_timestamp(seconds)?;
    let format = format_description::parse("[year][month][day]T[hour][minute][second]")?;

    Ok(format!("{}.{milliseconds:03}Z", datetime.format(&format)?))
}

fn run_label(name: Option<&str>, argv: &[String]) -> String {
    let label = name
        .map(sanitize_label)
        .unwrap_or_else(|| command_label(argv));

    if label.is_empty() {
        "run".to_string()
    } else {
        label
    }
}

fn command_label(argv: &[String]) -> String {
    let command = argv
        .first()
        .and_then(|value| Path::new(value).file_name())
        .and_then(|value| value.to_str())
        .unwrap_or("command");
    let command = sanitize_label(command);

    if let Some(code) = python_inline_code(argv) {
        let code = sanitize_label(code);

        if !code.is_empty() {
            return truncate_label(&format!("{command}-{code}"));
        }
    }

    command
}

fn python_inline_code(argv: &[String]) -> Option<&str> {
    let command = argv.first()?;

    if !Path::new(command)
        .file_name()?
        .to_str()?
        .to_ascii_lowercase()
        .contains("python")
    {
        return None;
    }

    argv.windows(2)
        .find(|pair| pair[0] == "-c")
        .map(|pair| pair[1].as_str())
}

fn sanitize_label(value: &str) -> String {
    let mut slug = String::new();

    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }

    truncate_label(slug.trim_matches('-'))
}

fn truncate_label(label: &str) -> String {
    label.chars().take(48).collect()
}
