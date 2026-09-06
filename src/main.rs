use std::path::PathBuf;

use clap::Parser;
use rrcounter::{Config, build_app, config};

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let config = args.to_config();

    let router = build_app(&config).await;

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", &config.server_port))
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();
}

#[derive(Parser)]
struct CliArgs {
    #[arg(long, default_value = config::DEFAULT_DB_PATH)]
    db_path: PathBuf,

    #[arg(long, default_value_t = config::DEFAULT_SERVER_PORT)]
    server_port: u16,
}

impl CliArgs {
    pub fn to_config(&self) -> Config {
        Config {
            db_path: self.db_path.to_str().unwrap().to_owned(),
            server_port: self.server_port,
        }
    }
}

#[test]
fn test_cli_all_args() {
    let args = CliArgs::parse_from(vec![
        "ignore__binary_name",
        "--db-path",
        "/path/to/my.db",
        "--server-port",
        "8080",
    ]);
    assert_eq!(PathBuf::from("/path/to/my.db"), PathBuf::from(args.db_path));
    assert_eq!(8080, args.server_port);
}

#[test]
fn test_cli_invalid_arg() {
    let arg_parse_result = CliArgs::try_parse_from(vec!["ignore__binary_name", "--invalid-arg"]);
    assert!(arg_parse_result.is_err_and(|e| e.kind() == clap::error::ErrorKind::UnknownArgument))
}
