# coldtrace Version Log

This log tracks `coldtrace` by project version and commit.

Commit is `pending` until the version is committed.

## 0.0.7 - Docs Split

Commit: pending
Date: 2026-06-23

### Changed

- Trimmed README to project summary, current behavior, usage, and doc links.
- Moved detailed C0 record shape, exit semantics, project layout, Lima workflow, non-goals, and `/proc` notes to `docs/c0-details.md`.
- Bumped crate version to `0.0.7`.

### Verified

- `cargo test`
- `limactl shell devbox -- bash -lc 'cd /Users/arjunpherwani/dev/coldtrace && cargo test'`

## 0.0.6 - README

Commit: pending
Date: 2026-06-22

### Added

- README with project purpose, current C0 behavior, usage examples, record shape, important exit semantics, project layout, Lima workflow, non-goals, and next `/proc` layer.

### Changed

- Bumped crate version to `0.0.6`.

### Verified

- `cargo test`
- `limactl shell devbox -- bash -lc 'cd /Users/arjunpherwani/dev/coldtrace && cargo test'`

## 0.0.5 - Readable Run IDs

Commit: pending
Date: 2026-06-21

### Changed

- Changed run directories from millisecond/pid-based names to UTC timestamp-based names.
- Run ids now use the shape `YYYYMMDDTHHMMSS.mmmZ-<label>`.
- `--name` is used directly as the run label after filename-safe sanitization.
- Unnamed Python `-c` runs derive labels from inline code, such as `python3-import-torch`.
- Run directory creation now handles timestamp collisions with numeric suffixes.

### Verified

- `cargo test`
- `limactl shell devbox -- bash -lc 'cd /Users/arjunpherwani/dev/coldtrace && cargo test'`

## 0.0.4 - Test Setup

Commit: pending
Date: 2026-06-21

### Added

- `src/lib.rs` so integration-style tests can import production modules.
- `tests/ut/` for cheap unit-style tests outside `src`.
- `tests/run_command.rs` for binary-level behavior tests.
- Coverage for CLI parsing, record serialization, stdout/stderr capture, child exit code `7`, and SIGTERM recording.

### Verified

- `cargo test`
- `limactl shell devbox -- bash -lc 'cd /Users/arjunpherwani/dev/coldtrace && cargo test'`

## 0.0.3 - Clap And Typed Records

Commit: pending
Date: 2026-06-21

### Added

- `clap` command parsing for `coldtrace run`.
- `--name` support.
- `serde` and `serde_json` for typed `record.json` output.
- Module split across `main`, `cli`, `runner`, and `record`.

### Changed

- Removed hand-written CLI parsing and hand-written JSON.
- Pinned crate versions for Lima's Rust/Cargo 1.75 toolchain.

## 0.0.2 - Basic Direct Run

Commit: pending
Date: 2026-06-21

### Added

- `coldtrace run -- <command> [args...]`.
- Per-run directories under `runs/<run-id>/`.
- `stdout.log`, `stderr.log`, and `record.json`.
- Parent-measured wall-clock duration.
- Child PID, exit code, Unix signal, argv, cwd, and stdout/stderr byte counts.

### Notes

- Nonzero child exits are valid run outcomes, not tracer failures.

## 0.0.1 - Cargo Init

Commit: pending
Date: 2026-06-21

### Added

- Initial Cargo binary crate for `coldtrace`.
- Baseline `.gitignore`.
