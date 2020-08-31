mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::Clap;

use args::{Opts, SubCommand};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Encode(e) => commands::encode(e),
        SubCommand::Decode(d) => commands::decode(d),
        SubCommand::Remove(r) => commands::remove(r),
        SubCommand::Print(p) => commands::print(p),
    }
}
