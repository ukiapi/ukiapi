use colored::Colorize;
use std::fs;
use std::process::{Command, Stdio};

/// Create a new UkiApi project with the given name.
/// This will run `cargo init`, add necessary dependencies to `Cargo.toml`,
/// and create a boilerplate `src/main.rs`.
pub fn run_new(name: String) {
    println!(
        "\n  {} Creating new UkiApi project: {}\n",
        "🚀".cyan(),
        name.bold()
    );

    // 1. cargo init
    let status = Command::new("cargo")
        .arg("init")
        .arg(&name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap_or_else(|_| {
            eprintln!("{} Failed to run cargo init", "error:".red().bold());
            std::process::exit(1);
        });

    if !status.success() {
        eprintln!("{} cargo init failed", "error:".red().bold());
        std::process::exit(1);
    }

    // 2. Update Cargo.toml
    let cargo_toml_path = format!("{}/Cargo.toml", name);
    let mut cargo_toml = fs::read_to_string(&cargo_toml_path).unwrap();

    let deps = r#"
ukiapi = { path = "../../ukiapi" }
ukiapi-macros = { path = "../../ukiapi-macros" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
validator = { version = "0.20", features = ["derive"] }
schemars = "0.8"

[package.metadata.cargo-machete]
ignored = ["schemars", "serde", "serde_json", "validator"]
"#;

    if !cargo_toml.contains("[dependencies]") {
        cargo_toml.push_str("\n[dependencies]\n");
    }
    cargo_toml.push_str(deps);
    fs::write(&cargo_toml_path, cargo_toml).unwrap();

    // 3. Create boilerplate main.rs
    let main_rs_path = format!("{}/src/main.rs", name);
    let main_rs_content = r#"use ukiapi::{routes, ValidatedJson, serve, Json};
use ukiapi::{get, post};
use serde::{Deserialize, Serialize};
use validator::Validate;
use schemars::JsonSchema;

#[derive(Clone)]
pub struct AppState {}

#[derive(Debug, Serialize, Deserialize, Validate, JsonSchema)]
pub struct HelloRequest {
    #[validate(length(min = 1))]
    pub name: String,
}

#[get("/hello")]
pub async fn hello() -> &'static str {
    "Hello from UkiApi!"
}

#[post("/greet")]
pub async fn greet(ValidatedJson(payload): ValidatedJson<HelloRequest>) -> Json<serde_json::Value> {
    Json(json!({ "message": format!("Hello, {}!", payload.name) }))
}

#[tokio::main]
async fn main() {
    let state = AppState {};

    routes![AppState,
        hello_route().with_state::<AppState>(),
        greet_route().with_state::<AppState>()
    ]
    .serve(state)
    .await;
}
"#;
    fs::write(&main_rs_path, main_rs_content).unwrap();

    println!(
        "  {} Project {} created successfully!",
        "✓".green().bold(),
        name.bold()
    );
    println!("  {} To get started:", "→".cyan());
    println!("    cd {}", name);
    println!("    uki dev\n");
}
