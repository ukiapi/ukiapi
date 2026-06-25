# rustapi-cli

The command-line interface for managing RustAPI projects.

## 🛠️ Installation

```bash
cargo install --path rustapi-cli
```

## 🚀 Commands

### `new`
Creates a new RustAPI project with a basic boilerplate.
```bash
rustapi new <project_name>
```

### `dev`
Runs the application in development mode.
- Sets `RUSTAPI_HOST` and `RUSTAPI_PORT` (default: `127.0.0.1:3000`).
- Optional `--reload` flag for hot-reloading (requires `cargo-watch`).
```bash
rustapi dev [--reload] [--port <port>]
```

### `run`
Runs the application in production mode (using `cargo run --release`).
```bash
rustapi run [--host <host>] [--port <port>]
```

## ⚙️ Environment Variables

The CLI automatically sets the following environment variables for your application:
- `RUSTAPI_HOST`: The host the server should bind to.
- `RUSTAPI_PORT`: The port the server should listen on.
- `TOKIO_WORKER_THREADS`: Set via the `--workers` flag.
