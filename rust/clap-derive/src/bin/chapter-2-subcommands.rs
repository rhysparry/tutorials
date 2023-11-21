use clap::{Args, Parser, Subcommand};

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
    Add(AddArgs),
}

#[derive(Args)]
struct AddArgs {
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(add_args) => {
            println!("Adding file: {:?}", add_args.name.as_deref());
        }
    }
}