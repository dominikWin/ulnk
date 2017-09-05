#[macro_use]
extern crate log;
extern crate env_logger;

mod args;
mod db;

use args::Args;
use db::Database;

fn main() {
    env_logger::init().unwrap();
    let args = Args::parse();
    info!("{:?}", args);
    let db = Database::new(&args).unwrap();
    db.init_db();

    db.close();
}
