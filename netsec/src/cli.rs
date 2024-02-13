use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct IpRange {
    // Need to parse string if valid ip address
    #[arg(short, long)]
    pub ip: Option<String>,

}