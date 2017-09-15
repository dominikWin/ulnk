#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate futures;

mod config;
mod db;
mod api_server;
mod args;

use config::Config;
use db::Database;
use args::Args;

fn main() {
    env_logger::init().unwrap();
    let config = Config::parse();
    let args = Args::new();
    info!("{:?}", config);
    info!("{:?}", args);
    let db = Database::new(&config).unwrap();
    if args.init_db {
        db.init_db();
        return;
    }

    api_server::start();

    db.close();
}
