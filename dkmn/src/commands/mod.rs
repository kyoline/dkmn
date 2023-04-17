use crate::prelude::*;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommand {
    Add { image: String },
    List,
    Remove { identifier: u32 },
}
