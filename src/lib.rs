use actix_web::{get, web, HttpResponse, Responder};

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
    cfg.service(hello);
    // Add other services here in the future
}
