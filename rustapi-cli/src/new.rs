use clap::Parser;
use rustapi_cli::run_new;

#[derive(Parser)]
#[command(name = "rustapi-new", about = "Create a new RustAPI project", version)]
struct Cli {
    /// Name of the project
    name: String,
}

fn main() {
    let cli = Cli::parse();
    run_new(cli.name);
}
