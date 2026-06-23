# C0 Details

This note holds detailed C0 behavior and implementation notes that do not need to live in the README.

## Current Record Shape

`record.json` currently contains the C0 direct-run facts:

```json
{
  "schema_version": "coldtrace.run.v0",
  "run_id": "20260622T022115.254Z-torch-import",
  "name": "torch-import",
  "launcher": {
    "mode": "direct",
    "unit_name": null
  },
  "command": {
    "argv": ["python3", "-c", "import torch"],
    "cwd": "/Users/arjunpherwani/dev/coldtrace"
  },
  "timing": {
    "started_unix_ms": 1782094875254,
    "wall_ms": 1234,
    "time_to_first_stdout_ms": null,
    "time_to_ready_ms": null
  },
  "exit": {
    "code": 0,
    "signal": null
  },
  "process": {
    "pid": 12345,
    "cgroup": null,
    "peak_rss_kb": null,
    "mapped_files_count": null
  },
  "output": {
    "stdout_bytes": 0,
    "stderr_bytes": 0
  },
  "artifacts": {
    "stdout": "stdout.log",
    "stderr": "stderr.log",
    "proc_status": null,
    "proc_cgroup": null,
    "proc_maps": null,
    "importtime": null,
    "strace_summary": null
  }
}
```

Some fields are intentionally `null`. They mark where later runtime layers will land.

## Important Semantics

A nonzero child exit is not a `coldtrace` failure.

For example:

```bash
cargo run -- run -- python3 -c 'import sys; sys.exit(7)'
```

should make `coldtrace` finish normally and record:

```json
"exit": {
  "code": 7,
  "signal": null
}
```

Exit code `7` is just the child program's numeric exit status. `coldtrace` does not assign universal meanings to child exit codes.

If the child is killed by a signal, the record has `code: null` and a signal number:

```json
"exit": {
  "code": null,
  "signal": 15
}
```

## Project Layout

```text
src/
  lib.rs       Library root for tests and shared modules
  main.rs      Binary entrypoint and command dispatch
  cli.rs       clap command definitions
  runner.rs    Process spawning, timing, stdout/stderr capture
  record.rs    Typed record.json model
tests/
  run_command.rs   Binary-level behavior tests
  ut.rs            Test harness for cheap unit-style tests
  ut/
    cli.rs
    record.rs
```

`src/lib.rs` exists so tests under `tests/` can import production modules as `coldtrace::runner`, `coldtrace::record`, and so on.

## Development Environment

This project is meant to run inside a Lima Ubuntu VM on macOS.

The `devbox` VM mounts this repo at:

```text
/Users/arjunpherwani/dev/coldtrace
```

Run tests locally:

```bash
cargo test
```

Run tests inside Lima:

```bash
limactl shell devbox -- bash -lc 'cd /Users/arjunpherwani/dev/coldtrace && cargo test'
```

Crate versions are pinned because the Lima VM currently uses Rust/Cargo 1.75.

## Non-Goals For Now

`coldtrace` is not currently:

- a container runtime
- a daemon
- a web dashboard
- a custom tracing engine
- an eBPF framework
- a GPU scheduler
- a Modal clone

## Next Layer

The next learning layer is live `/proc/<pid>` snapshots.

The key race is that `/proc/<pid>` exists only while the process exists. For a short-lived child, the parent may miss the window to read files like:

```text
/proc/<pid>/status
/proc/<pid>/cgroup
/proc/<pid>/maps
/proc/<pid>/fd
```

So the first manual experiment should use a deliberately long-lived child:

```bash
python3 -c 'import time; print("ready"); time.sleep(10)'
```

Then `coldtrace` can translate that into a small Rust implementation that snapshots selected `/proc` files before calling `wait()`.
