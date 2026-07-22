use std::sync::Mutex;

use rusqlite::{params, Connection, Result as SqlResult};

// TODO: include building, destruction and any other kind of stateful tile data in the persistence layer
pub struct WorldPersistence {
    conn: Mutex<Connection>,
}

impl WorldPersistence {
    pub fn new(db_path: &str) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;
        let persistence = Self {
            conn: Mutex::new(conn),
        };
        persistence.init_schema()?;
        Ok(persistence)
    }

    fn init_schema(&self) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS world_config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            ",
        )?;
        Ok(())
    }

    pub fn load_world_seed(&self) -> SqlResult<u64> {
        let conn = self.conn.lock().unwrap();
        let result: String = conn.query_row(
            "SELECT value FROM world_config WHERE key = 'seed'",
            [],
            |row| row.get(0),
        )?;
        result
            .parse::<u64>()
            .map_err(|e| rusqlite::Error::InvalidParameterName(format!("Invalid seed: {}", e)))
    }

    pub fn save_world_seed(&self, seed: u64) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO world_config (key, value) VALUES ('seed', ?1)",
            params![seed.to_string()],
        )?;
        Ok(())
    }
}
