use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
pub struct Opt {
    #[clap(env)]
    pub cert: PathBuf,

    #[clap(env)]
    pub key: PathBuf,
}