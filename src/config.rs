pub const DEFAULT_DB_PATH: &str = "visits.db";
pub const DEFAULT_SERVER_PORT: u16 = 3000;

pub struct Config {
    pub db_path: String,
    pub server_port: u16,
}

impl Config {
    pub fn default() -> Config {
        Config {
            db_path: DEFAULT_DB_PATH.to_owned(),
            server_port: DEFAULT_SERVER_PORT,
        }
    }
}
