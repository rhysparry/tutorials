use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Network port to listen on
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,
}
fn main() {
    let cli = Cli::parse();

    println!("port: {:?}", cli.port);
}