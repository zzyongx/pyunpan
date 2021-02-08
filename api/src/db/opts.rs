use clap::Clap;

#[derive(Clap)]
pub struct Opts {
    #[clap(long, default_value = "sqlite://pyunpan.db")]
    pub path: String,

    #[clap(long)]
    pub op: String,
}
