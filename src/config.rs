use std::{net::Ipv4Addr, str::FromStr};

pub const DEFAULT_DB_PATH: &str = "visits.db";
pub const DEFAULT_SERVER_PORT: u16 = 3000;
pub const DEFAULT_SERVER_BIND_IP: &str = "127.0.0.1";

pub struct Config {
    pub db_path: String,
    pub server_port: u16,
    pub server_bind_ip: Ipv4Addr,
}

impl Config {
    pub fn default() -> Config {
        Config {
            db_path: DEFAULT_DB_PATH.to_owned(),
            server_port: DEFAULT_SERVER_PORT,
            server_bind_ip: Ipv4Addr::from_str(DEFAULT_SERVER_BIND_IP).unwrap(),
        }
    }
}

/* ==== UNIT TESTS ==== */
#[cfg(test)]
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_default_config() -> TestResult {
    let cfg = Config::default();

    assert_eq!(cfg.db_path, "visits.db");
    assert_eq!(cfg.server_port, 3000);
    assert_eq!(cfg.server_bind_ip, Ipv4Addr::from_str("127.0.0.1")?);
    Ok(())
}
