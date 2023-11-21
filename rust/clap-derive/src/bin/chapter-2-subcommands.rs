use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add { name: Option<String> },
}
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { name } => {
            println!("Adding file: {:?}", name.as_deref());
        }
    }
}