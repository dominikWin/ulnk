#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate futures;

mod config;
mod db;
mod api_server;

use config::Config;
use db::Database;

fn main() {
    env_logger::init().unwrap();
    let config = Config::parse();
    info!("{:?}", config);
    let db = Database::new(&config).unwrap();
    // db.init_db();

    api_server::start();

    db.close();
}
