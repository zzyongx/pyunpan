#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod opts;
use crate::opts::{Opts, SubCommand};

mod db;
mod web;

fn main() {
    let opts = Opts::new();

    match opts.subcmd {
        SubCommand::Db(o) => db::run(o),
        SubCommand::Rest(o) => web::run(o),
    }
}
