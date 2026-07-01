# uki

The command-line interface for managing Ukidama projects.

## 🛠️ Installation

```bash
cargo install --path uki
```

## 🚀 Commands

### `new`
Creates a new Ukidama project with a basic boilerplate.
```bash
uki new <project_name>
```

### `dev`
Runs the application in development mode.
- Sets `UKIDAMA_HOST` and `UKIDAMA_PORT` (default: `127.0.0.1:3000`).
- Optional `--reload` flag for hot-reloading (requires `cargo-watch`).
```bash
uki dev [--reload] [--port <port>]
```

### `run`
Runs the application in production mode (using `cargo run --release`).
```bash
uki run [--host <host>] [--port <port>]
```

## ⚙️ Environment Variables

The CLI automatically sets the following environment variables for your application:
- `UKIDAMA_HOST`: The host the server should bind to.
- `UKIDAMA_PORT`: The port the server should listen on.
- `TOKIO_WORKER_THREADS`: Set via the `--workers` flag.
