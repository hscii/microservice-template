use actix_web::{test, App};
use service::hello;

#[actix_rt::test]
async fn test_hello() {
    let mut app = test::init_service(App::new().service(hello)).await;
    let req = test::TestRequest::get().uri("/hello").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, "hello world");
}
