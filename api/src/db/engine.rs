use crate::db::Opts;

type DbError = String;

pub trait DbEngine {
    fn init(self: &mut Self, opt: &Opts) -> Result<(), String>;
    fn dump(self: &mut Self, opt: &Opts) -> Result<(), String>;
}
