pub const DEFAULT_DB_PATH: &str = "visits.db";
pub const DEFAULT_SERVER_PORT: u16 = 3000;
pub const DEFAULT_SERVER_BIND_IP: &str = "127.0.0.1";

pub struct Config {
    pub db_path: String,
    pub server_port: u16,
    pub server_bind_ip: String,
}

impl Config {
    pub fn default() -> Config {
        Config {
            db_path: DEFAULT_DB_PATH.to_owned(),
            server_port: DEFAULT_SERVER_PORT,
            server_bind_ip: DEFAULT_SERVER_BIND_IP.to_owned(),
        }
    }
}

#[test]
fn test_default_config() {
    let cfg = Config::default();

    assert_eq!(cfg.db_path, "visits.db");
    assert_eq!(cfg.server_port, 3000);
    assert_eq!(cfg.server_bind_ip, "127.0.0.1");
}
