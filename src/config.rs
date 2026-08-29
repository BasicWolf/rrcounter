pub struct Config {
    pub db_path: String,
    pub server_port: u16,
}

impl Config {
    pub fn default() -> Config {
        Config {
            db_path: "visits.db".to_owned(),
            server_port: 3000,
        }
    }
}
