use actix_web::{delete, get, patch, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new())
        .bind("127.0.0.1:8080")? // Desired IP and port
        .run()
        .await
}
