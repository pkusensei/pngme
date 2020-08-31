use clap::Clap;
use std::path::PathBuf;

#[derive(Clap)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Encode(Encode),
    Decode(Decode),
    Remove(Remove),
    Print(Print),
}

#[derive(Clap)]
pub struct Encode {
    #[clap(parse(from_os_str))]
    pub path: PathBuf,

    #[clap(parse(from_str))]
    pub chunk_type: String,

    #[clap(parse(from_str))]
    pub message: String,
}

#[derive(Clap)]
pub struct Decode {
    #[clap(parse(from_os_str))]
    pub path: PathBuf,

    #[clap(parse(from_str))]
    pub chunk_type: String,
}

#[derive(Clap)]
pub struct Remove {
    #[clap(parse(from_os_str))]
    pub path: PathBuf,

    #[clap(parse(from_str))]
    pub chunk_type: String,
}

#[derive(Clap)]
pub struct Print {
    #[clap(parse(from_os_str))]
    pub path: PathBuf,
}
