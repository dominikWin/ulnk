extern crate postgres;

use self::postgres::{Connection, TlsMode};
use args::Args;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(args: &Args) -> Option<Database> {
        let connect_params: String = format!("postgresql://{}:{}@{}/{}", args.dbuname, args.dbupasswd, args.dbpath, args.dbname);
        let cp = connect_params.clone(); // For logging
        let conn = Connection::connect(connect_params, TlsMode::None);
        if conn.is_err() {
            error!("Failed to connect to database {}", cp);
            return Option::None
        }
        let conn = conn.unwrap();
        info!("Connected to database {}", cp);

        let db = Database {conn: conn};
        Option::Some(db)
    }

    pub fn close(self) {
        self.conn.finish().unwrap();
        info!("Closed connection to database");
    }
}
