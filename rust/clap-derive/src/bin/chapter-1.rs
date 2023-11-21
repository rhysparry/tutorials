use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)] // Read the author, version, and about from Cargo.toml
struct Cli {
    #[arg(long)]
    two: String,
    #[arg(long)]
    one: String,
}

fn main() {
    let cli = Cli::parse();

    println!("one: {}", cli.one);
    println!("two: {}", cli.two);
}