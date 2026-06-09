use clap::Subcommand;
use std::path::PathBuf;

use clap::Parser;

mod crypto;
mod pack;
mod unpack;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    pack {
        #[arg(short, long)]
        password: Option<String>,
        files: Vec<PathBuf>,
    },

    unpack {
        #[arg(short, long)]
        password: Option<String>,
        archive: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    println!("{:#?}", cli);
}
