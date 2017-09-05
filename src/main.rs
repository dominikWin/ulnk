#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate futures;

mod args;
mod db;
mod api_server;

use args::Args;
use db::Database;

fn main() {
    env_logger::init().unwrap();
    let args = Args::parse();
    info!("{:?}", args);
    let db = Database::new(&args).unwrap();
    db.init_db();

    api_server::start();

    db.close();
}
