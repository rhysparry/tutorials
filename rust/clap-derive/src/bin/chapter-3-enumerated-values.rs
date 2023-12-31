use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// What mode to run the program in
    #[arg(value_enum)]
    mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Run swiftly
    Fast,
    /// Crawl slowly but steadily
    ///
    /// This paragraph is ignored because there is not long help text for possible values.
    Slow,
}

fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::Fast => println!("Running fast"),
        Mode::Slow => println!("Running slow"),
    }
}