extern crate postgres;

use self::postgres::{Connection, TlsMode};
use config::Config;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(config: &Config) -> Option<Database> {
        let connect_params: String = format!("postgresql://{}:{}@{}/{}", config.dbuname, config.dbupasswd, config.dbpath, config.dbname);
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

    pub fn init_db(self) {
        self.exec_force("CREATE TABLE IF NOT EXISTS refs (
                        id     SERIAL PRIMARY KEY,
                        shortl VARCHAR[20] NOT NULL,
                        longl  VARCHAR[512] NOT NULL
                        );");
        self.exec_force("CREATE UNIQUE INDEX IF NOT EXISTS shortl_idx ON refs (shortl);");
    }

    fn exec_force(&self, cmd: &str) {
        if let Err(e) = self.conn.execute(cmd, &[]) {
            panic!("Error running initialization command {}: {}", cmd, e);
        }
    }
}
