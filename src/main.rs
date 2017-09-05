#[macro_use]
extern crate log;
extern crate env_logger;

mod args;

fn main() {
    env_logger::init().unwrap();
    info!("{:?}", args::Args::parse());
}
