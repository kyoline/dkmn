use clap::Parser;
use commands::SubCommand;
use prelude::Error;

mod commands;
mod error;
mod prelude;

#[derive(Parser, Debug)]
#[clap(version, about)]
/// docker manager for home servers.
struct Args {
    #[clap(subcommand)]
    cmd: SubCommand,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    match args.cmd {
        SubCommand::Add { image } => {
            println!("add {}", image);
        }
        SubCommand::Remove { identifier } => {
            println!("remove {}", identifier);
        }
        SubCommand::List => {
            println!("list");
        }
    }

    Ok(())
}
