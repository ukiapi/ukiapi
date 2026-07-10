use clap::Parser;
use ukiapi_cli::run_new;

#[derive(Parser)]
#[command(name = "ukiapi-new", about = "Create a new UkiApi project", version)]
struct Cli {
    /// Name of the project
    name: String,
}

fn main() {
    let cli = Cli::parse();
    run_new(cli.name);
}
