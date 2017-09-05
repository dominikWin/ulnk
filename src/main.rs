#[macro_use]
extern crate log;
extern crate env_logger;
extern crate postgres;

mod args;
mod db;

use db::Database;

fn main() {
    env_logger::init().unwrap();
    let args = args::Args::parse();
    info!("{:?}", args);
    let db = Database::new(&args).unwrap();

    db.close();
}
