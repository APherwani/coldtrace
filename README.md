# coldtrace

`coldtrace` is a small Rust CLI for learning Linux process/runtime behavior by tracing cold starts of ML-ish workloads.

The first target workload is:

```bash
python3 -c 'import torch'
```

Current version: `0.0.7`

## What It Does Today

`coldtrace` is currently a direct process wrapper.

At a high level, it:

1. Parses the child command after `--`.
2. Creates a run directory under `runs/`.
3. Starts the child process.
4. Redirects the child's stdout and stderr into log files.
5. Measures wall-clock time from the parent process.
6. Waits for the child to exit.
7. Records the child PID, exit code or signal, argv, cwd, output byte counts, and artifact paths.
8. Writes everything into `record.json`.

That means this:

```bash
coldtrace run -- python3 -c 'print("hello")'
```

creates something like:

```text
runs/
  20260622T022115.254Z-python3-print-hello/
    record.json
    stdout.log
    stderr.log
```

The tool is intentionally boring at this stage. It is not trying to be a profiler yet. It is building the reliable parent-process measurement foundation first.

## Usage

From the Lima VM:

```bash
cd /Users/arjunpherwani/dev/coldtrace
cargo run -- run -- python3 -c 'print("hello")'
```

With an explicit run name:

```bash
cargo run -- run --name torch-import -- python3 -c 'import torch'
```

The CLI prints the run directory:

```text
runs/20260622T022115.254Z-torch-import
```

Inspect the artifacts:

```bash
cat runs/<run-id>/stdout.log
cat runs/<run-id>/stderr.log
python3 -m json.tool runs/<run-id>/record.json
```

## Docs

- [C0 details](docs/c0-details.md)
- [Version log](VERSION_LOG.md)
