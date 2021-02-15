use crate::web::util::ApiToken;
use clap::Clap;
use rocket;
use std::path::PathBuf;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/file/<file..>?<md5>&<rewrite>")]
fn put_file(
    token: ApiToken,
    file: PathBuf,
    md5: Option<String>,
    rewrite: Option<bool>,
) -> Result<String, rocket::http::Status> {
    Ok(format!("{:?} {:?} {:?}", token.uid, token.value, file))
}

#[derive(Clap)]
pub struct Opts {
    #[clap(long, default_value = "8000")]
    _port: i32,
}

pub fn run(_opt: Opts) {
    rocket::ignite()
        .mount("/", routes![index, put_file])
        .launch();
}
