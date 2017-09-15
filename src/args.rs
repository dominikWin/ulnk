extern crate clap;

use self::clap::{Arg, App};

#[derive(Debug)]
pub struct Args {
    pub init_db: bool,
}

impl Args {
    pub fn new() -> Self {
        let matches = App::new("ulnk")
                        .version("0.0.1")
                        .author("Dominik Winecki")
                        .arg(Arg::with_name("initdb")
                             .short("i")
                             .long("initdb")
                             .help("Perform initial database setup")
                             .takes_value(false))
                        .get_matches();
        let init_db = matches.is_present("initdb");
        Args {init_db: init_db}
    }
}
