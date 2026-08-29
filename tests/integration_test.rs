mod common;

use common::SUT;

#[tokio::test]
async fn post_then_get_returns_count() {
    let sut = SUT::new().await;
    sut.post_visit().await;
    assert_eq!(1, sut.get_visits().await);
}
