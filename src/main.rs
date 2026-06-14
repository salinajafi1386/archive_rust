use clap::Subcommand;
use std::path::PathBuf;

use clap::Parser;

use pack::pack;
use unpack::unpack;

mod crypto;
mod pack;
mod unpack;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Pack {
        #[arg(short, long)]
        output: PathBuf,

        #[arg(short, long)]
        password: Option<String>,

        files: Vec<PathBuf>,
    },

    Unpack {
        #[arg(short, long)]
        password: Option<String>,

        archive: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Pack {
            files,
            password,
            output,
        } => {
            if let Err(e) = pack(files.clone(), password.clone(), output.clone()) {
                eprintln!("Error: {}", e);
            }
        }

        Commands::Unpack { archive, password } => {
            if let Err(e) = unpack(archive.clone(), password.clone()) {
                eprintln!("Error: {}", e);
            }
        }
    }
}
