use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "qr-transfer", about = "transfer files over wifi")]
pub struct Opt {
    #[structopt(help = "Input files or directories", parse(from_os_str))]
    pub files: Vec<PathBuf>,

    #[structopt(short = "d", long = "debug")]
    pub debug: bool,

    #[structopt(short = "z", long = "zip")]
    pub zip: bool,
}