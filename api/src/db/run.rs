use crate::db::*;

pub fn run(opt: Opts) {
    let mut db_engine: Box<dyn DbEngine>;
    if opt.path.starts_with("sqlite://") {
        db_engine = Box::new(Sqlite {});
    } else {
        panic!("unknow path {}", opt.path);
    }

    let rc;
    if opt.op == "init" {
        rc = db_engine.init(&opt);
    } else if opt.op == "dump" {
        rc = db_engine.dump(&opt);
    } else {
        panic!("unknow op {}", opt.op);
    }

    if rc.is_err() {
        println!("{:?}", rc)
    }
}
