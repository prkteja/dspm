use clap::{Args, Parser, Subcommand};
use super::constants;

#[derive(Debug, Parser)]
#[command(name = constants::COMMAND)]
#[command(about = constants::ABOUT, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init {
        #[arg(long, default_value_t=constants::DEFAULT_KEY_SIZE)]
        key_size: u32,
    },

    List {
        #[arg(short, long)]
        domain: Option<String>,
    },

    Add(AddArgs),
    Show(AddArgs)
}

#[derive(Debug, Args)]
pub struct AddArgs {
    pub domain: String,
    pub username: String
}
