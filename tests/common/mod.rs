use std::error::Error;

use axum::{
    Router,
    body::{Body, to_bytes},
    http::Request,
};
use rrcounter::{Config, VisitsResponse, build_app};
use tempfile::Builder;
use tower::ServiceExt; // for Router.oneshot

pub struct SUT {
    app: Router,
}

impl SUT {
    pub async fn new() -> SUT {
        SUT {
            app: SUT::build_app().await,
        }
    }

    async fn build_app() -> Router {
        let temp_db_file = Builder::new()
            .prefix("rrcounter_test")
            .suffix(".db")
            .tempfile()
            .expect("Cannot construct temporary file for test database");

        let temp_db_path = temp_db_file
            .path()
            .to_str()
            .expect("Cannot get temporary test database path")
            .to_string();

        let config = Config {
            db_path: temp_db_path,
            server_port: 3000,
        };

        build_app(&config).await
    }

    pub async fn get_status_page(&self) -> Result<String, Box<dyn Error>> {
        let resp = self
            .app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/status")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await?;

        let bytes = to_bytes(resp.into_body(), usize::MAX).await?;
        let body_as_string = String::from_utf8(bytes.to_vec())?;
        Ok(body_as_string)
    }

    pub async fn post_visit(&self) {
        self.app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/visit")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
    }

    pub async fn get_visits(&self) -> i64 {
        // GET the count
        let resp = self
            .app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/visits")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: VisitsResponse = serde_json::from_slice(&bytes).unwrap();
        body.visits
    }
}
