use actix_web::{delete, get, patch, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use supabase_actix_auth_middleware::jwt_middleware;

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let auth = HttpAuthentication::bearer(jwt_middleware);
        App::new().wrap(auth).service(ping)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
