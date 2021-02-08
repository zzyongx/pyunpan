use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "zzyongx 461193892@qq.com")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

impl Opts {
    pub fn new() -> Opts {
        return Opts::parse();
    }
}

use crate::db;
use crate::web;

#[derive(Clap)]
pub enum SubCommand {
    Rest(web::Opts),
    Db(db::Opts),
}
