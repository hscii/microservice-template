use actix_web::{test, App};
use service::endpoints::ping::ping;
#[actix_rt::test]

async fn test_ping() {
    // 1. Initialize the service
    let mut app = test::init_service(App::new().service(ping)).await;

    // 2. Create a request
    let req = test::TestRequest::get().uri("/ping").to_request();
    let resp = test::call_service(&mut app, req).await;

    // 3. Assert the response
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    assert_eq!(body, "pong");
}
