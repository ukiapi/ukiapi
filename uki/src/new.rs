use clap::Parser;
use uki::run_new;

#[derive(Parser)]
#[command(name = "uki-new", about = "Create a new Ukidama project", version)]
struct Cli {
    /// Name of the project
    name: String,
}

fn main() {
    let cli = Cli::parse();
    run_new(cli.name);
}
