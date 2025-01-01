// args.rs
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short = 'd', long = "directory")]
    pub directory: Option<String>,
    #[arg(short = 'r', long = "reversible")]
    pub reversible: Option<bool>,
}