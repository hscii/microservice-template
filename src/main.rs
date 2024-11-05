use actix_web::{get, web, HttpResponse, Responder};
use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use supabase_actix_auth_middleware::jwt_middleware;

/// Tests for the `/ping` endpoint
///
/// ```
/// use actix_web::{test, App};
/// use template::ping;
///
/// #[actix_rt::test]
/// async fn test_ping() {
///     let mut app = test::init_service(App::new().service(ping)).await;
///     let req = test::TestRequest::get().uri("/ping").to_request();
///     let resp = test::call_service(&mut app, req).await;
///     assert!(resp.status().is_success());
///     let body = test::read_body(resp).await;
///     assert_eq!(body, "pong");
/// }
/// ```
#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(jwt_middleware);
        App::new().wrap(auth).service(ping).service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
