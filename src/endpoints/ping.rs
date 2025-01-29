use actix_web::{get, HttpResponse, Responder};
use log::debug;

#[get("/ping")]
pub async fn ping() -> impl Responder {
    debug!("pong response served");
    HttpResponse::Ok().body("pong")
}
