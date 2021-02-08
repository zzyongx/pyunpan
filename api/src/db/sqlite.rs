use crate::db::{DbEngine, Opts};
use rusqlite::Connection;
use rusqlite::NO_PARAMS;

pub struct Sqlite {}

impl Sqlite {
    fn init_impl(self: &mut Self, opt: &Opts) -> rusqlite::Result<()> {
        let dbfile = &opt.path[9..];
        let conn = Connection::open(dbfile.to_string())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS pyp_files(
               id INTEGER PRIMARY KEY,
               root TEXT NOT NULL,
               fname TEXT NOT NULL,
               md5 TEXT NOT NULL)",
            NO_PARAMS,
        )?;

        return Ok(());
    }
}

impl DbEngine for Sqlite {
    fn init(self: &mut Self, opt: &Opts) -> Result<(), String> {
        let err = self.init_impl(opt);

        if err.is_err() {
            println!("{:?}", err);
            return Err("error".to_string());
        } else {
            return Ok(());
        }
    }

    fn dump(self: &mut Self, _opt: &Opts) -> Result<(), String> {
        return Ok(());
    }
}
