use actix_web::{web, App, HttpResponse, HttpServer, Responder};

pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ping", web::get().to(ping)))
        .bind("127.0.0.1:8080")? // Desired IP and port
        .run()
        .await
}
