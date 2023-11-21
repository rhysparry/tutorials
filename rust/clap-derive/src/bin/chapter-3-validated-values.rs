use std::ops::RangeInclusive;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Network port to listen on
    #[arg(value_parser = port_in_range)]
    port: u16,
}

fn main() {
    let cli = Cli::parse();

    println!("port: {:?}", cli.port);
}

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

fn port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("{} is not a valid port number", s))?;
    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!("Port not in range {}-{}", PORT_RANGE.start(), PORT_RANGE.end()))
    }
}