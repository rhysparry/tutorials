use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)] // Read the author, version, and about from Cargo.toml
#[command(next_line_help = true)] // Display the help message on the next line
struct Cli {
    /// The value for the two
    #[arg(long)]
    two: String,
    /// The value for the one
    #[arg(long)]
    one: String,
}

fn main() {
    let cli = Cli::parse();

    println!("one: {}", cli.one);
    println!("two: {}", cli.two);
}