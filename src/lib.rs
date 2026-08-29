mod database;

use std::sync::{
    Arc,
    atomic::{AtomicI64, Ordering},
};
use std::time::Duration;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};

use database::Database;
use serde::{Deserialize, Serialize};

// Shared app state: the lively counter plus the SQLite path (sendable clone).

// Notice the handlers take the state by value, not by reference.
// axum clones the state for each request (that's why AppState derives Clone).
// To avoid copying the counter on every clone, the heavy data lives inside Arcs.
#[derive(Clone)]
struct AppState {
    // SQLite3 supports Signed 8-byte integers, hence I64
    counter: Arc<AtomicI64>,
    flushed_counter: Arc<AtomicI64>,

    // We pass DB Path instead of structure with a connection
    // since ruslite Connections are not thread-safe
    // (i.e. they're not Send nor Sync)
    db_path: Arc<String>,
}

#[derive(Serialize, Deserialize)]
pub struct VisitsResponse {
    pub visits: i64,
}

pub async fn build_app() -> Router {
    let db_path = "visits.db".to_string();
    let db = Database::new(&db_path);

    let initial_counter_value = db.get_initial_value();
    let state = AppState {
        counter: Arc::new(AtomicI64::new(initial_counter_value)),
        flushed_counter: Arc::new(AtomicI64::new(initial_counter_value)),
        db_path: Arc::new(db_path),
    };

    // Spawn the periodic flusher.
    tokio::spawn(flusher(state.clone(), Duration::from_secs(5)));

    Router::new()
        .route("/visits", get(get_visits))
        .route("/visit", post(record_visit))
        .with_state(state)
}

async fn record_visit(State(state): State<AppState>) -> StatusCode {
    state.counter.fetch_add(1, Ordering::Relaxed);
    StatusCode::CREATED
}

async fn get_visits(State(state): State<AppState>) -> Json<VisitsResponse> {
    Json(VisitsResponse {
        visits: state.counter.load(Ordering::Relaxed),
    })
}

async fn flusher(state: AppState, interval: Duration) {
    let mut ticker = tokio::time::interval(interval);
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    loop {
        ticker.tick().await;

        let counter = state.counter.load(Ordering::Relaxed);

        match Database::new(&state.db_path).persist_counter(counter) {
            Ok(()) => {
                state.flushed_counter.swap(counter, Ordering::Relaxed);
            }
            Err(e) => {
                eprintln!("flush failed: {e}");
            }
        }
    }
}
