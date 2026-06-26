use clap::{Parser, Subcommand};
use colored::Colorize;
use rustapi_cli::run_new;
use std::env;
use std::process::{Command, Stdio};

// ─── CLI definition ──────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(
    name = "rustapi",
    about = "RustAPI CLI — develop and run your RustAPI app",
    version,
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new RustAPI project
    New {
        /// Name of the project
        name: String,
    },

    /// Run the app in development mode (debug build, optional hot-reload)
    Dev {
        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// Port to listen on
        #[arg(long, short, default_value_t = 3000)]
        port: u16,

        /// Number of Tokio worker threads (default: number of CPU cores)
        #[arg(long, default_value_t = 0)]
        workers: usize,

        /// Enable hot-reload via cargo-watch (recompiles on file changes)
        #[arg(long)]
        reload: bool,
    },

    /// Run the app in production mode (release build)
    Run {
        /// Host to bind to
        #[arg(long, default_value = "0.0.0.0")]
        host: String,

        /// Port to listen on
        #[arg(long, short, default_value_t = 3000)]
        port: u16,

        /// Number of Tokio worker threads (default: number of CPU cores)
        #[arg(long, default_value_t = 0)]
        workers: usize,
    },
}

// ─── Entry point ─────────────────────────────────────────────────────────────

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => {
            run_new(name);
        }
        Commands::Dev {
            host,
            port,
            workers,
            reload,
        } => {
            print_banner("dev", &host, port, reload);
            run_dev(host, port, workers, reload);
        }
        Commands::Run {
            host,
            port,
            workers,
        } => {
            print_banner("run", &host, port, false);
            run_release(host, port, workers);
        }
    }
}

// ─── Banners ─────────────────────────────────────────────────────────────────

fn print_banner(mode: &str, host: &str, port: u16, reload: bool) {
    println!();
    println!(
        "  {}  {}",
        "🦀 RustAPI".bold(),
        format!("v{}", env!("CARGO_PKG_VERSION")).dimmed()
    );
    println!(
        "  {}  {}",
        "mode:".dimmed(),
        if mode == "dev" {
            "development".yellow().bold()
        } else {
            "production".green().bold()
        }
    );
    println!(
        "  {}  {}",
        "addr:".dimmed(),
        format!("http://{}:{}", host, port).cyan().bold()
    );
    if reload {
        println!("  {}  {}", "reload:".dimmed(), "enabled".green());
    }
    if mode == "dev" {
        println!(
            "  {}  {}",
            "docs:".dimmed(),
            format!("http://{}:{}/docs", host, port).cyan()
        );
    }
    println!();
}

// ─── Env helpers ─────────────────────────────────────────────────────────────

/// Build the list of env vars to inject into cargo run / cargo watch.
fn build_env(host: &str, port: u16, workers: usize) -> Vec<(String, String)> {
    let mut env = vec![
        ("RUSTAPI_HOST".into(), host.to_string()),
        ("RUSTAPI_PORT".into(), port.to_string()),
    ];
    if workers > 0 {
        env.push(("TOKIO_WORKER_THREADS".into(), workers.to_string()));
    }
    env
}

// ─── Dev runner (debug build, optional hot-reload) ───────────────────────────

fn detect_bin() -> String {
    let output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .output()
        .unwrap_or_else(|e| {
            eprintln!(
                "{} Failed to run cargo metadata: {}",
                "error:".red().bold(),
                e
            );
            std::process::exit(1);
        });

    if !output.status.success() {
        eprintln!("{} cargo metadata failed", "error:".red().bold());
        std::process::exit(1);
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|e| {
        eprintln!(
            "{} Failed to parse cargo metadata: {}",
            "error:".red().bold(),
            e
        );
        std::process::exit(1);
    });

    let packages = json["packages"].as_array().unwrap();
    let dir_name = env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_default();

    let cli_bins = ["rustapi", "rustapi-new"];

    let mut bins: Vec<String> = Vec::new();
    for pkg in packages {
        if let Some(targets) = pkg["targets"].as_array() {
            for target in targets {
                if target["kind"]
                    .as_array()
                    .is_some_and(|k| k.iter().any(|v| v == "bin"))
                {
                    if let Some(name) = target["name"].as_str() {
                        if !cli_bins.contains(&name) {
                            bins.push(name.to_string());
                        }
                    }
                }
            }
        }
    }

    if bins.is_empty() {
        eprintln!("{} No binary targets found", "error:".red().bold());
        std::process::exit(1);
    }

    if bins.len() == 1 {
        return bins.into_iter().next().unwrap();
    }

    // Multiple bins: prefer one matching directory name, then "example"
    if let Some(idx) = bins.iter().position(|b| *b == dir_name) {
        return bins.swap_remove(idx);
    }
    if let Some(idx) = bins.iter().position(|b| *b == "example") {
        return bins.swap_remove(idx);
    }

    bins.into_iter().next().unwrap()
}

fn run_dev(host: String, port: u16, workers: usize, reload: bool) {
    let env = build_env(&host, port, workers);
    let bin = detect_bin();

    if reload {
        ensure_cargo_watch();

        let status = Command::new("cargo")
            .arg("watch")
            .arg("-x")
            .arg(format!("run --bin {bin}"))
            .envs(env)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .unwrap_or_else(|e| {
                eprintln!("{} {}", "error:".red().bold(), e);
                std::process::exit(1);
            });

        std::process::exit(status.code().unwrap_or(1));
    } else {
        let status = Command::new("cargo")
            .args(["run", "--bin", &bin])
            .envs(env)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .unwrap_or_else(|e| {
                eprintln!("{} {}", "error:".red().bold(), e);
                std::process::exit(1);
            });

        std::process::exit(status.code().unwrap_or(1));
    }
}

// ─── Production runner (release build) ───────────────────────────────────────

fn run_release(host: String, port: u16, workers: usize) {
    let env = build_env(&host, port, workers);
    let bin = detect_bin();

    let status = Command::new("cargo")
        .args(["run", "--release", "--bin", &bin])
        .envs(env)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap_or_else(|e| {
            eprintln!("{} {}", "error:".red().bold(), e);
            std::process::exit(1);
        });

    std::process::exit(status.code().unwrap_or(1));
}

// ─── cargo-watch guard ───────────────────────────────────────────────────────

fn ensure_cargo_watch() {
    let found = Command::new("cargo-watch")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok();

    if found {
        return;
    }

    eprintln!(
        "\n  {} {} is not installed.",
        "✗".red().bold(),
        "cargo-watch".yellow()
    );
    eprintln!(
        "  {} Run {} to install it, then try again.\n",
        "→".cyan(),
        "cargo install cargo-watch".bold()
    );
    std::process::exit(1);
}
