mod common;

use std::error::Error;

use common::SUT;

#[tokio::test]
async fn get_status() -> Result<(), Box<dyn Error>> {
    let sut = SUT::new().await;
    let status = sut.get_status_page().await?;
    assert_eq!("<h1>RRCounter is up and running!</h1>", status);
    Ok(())
}

#[tokio::test]
async fn post_then_get_returns_count() {
    let sut = SUT::new().await;
    sut.post_visit().await;
    assert_eq!(1, sut.get_visits().await);
}
