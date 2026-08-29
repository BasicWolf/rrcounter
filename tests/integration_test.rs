use axum::{body::Body, extract::Request};
use rrcounter::{VisitsResponse, build_app};
use tower::ServiceExt; // for oneshot

#[tokio::test]
async fn post_then_get_returns_count() {
    let app = build_app().await;

    // POST a visit
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/visit")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // GET the count
    let resp = app
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
    assert_eq!(body.visits, 1);
}
