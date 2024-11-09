use actix_web::{get, web, HttpResponse, Responder};
use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use supabase_actix_auth_middleware::jwt_middleware;

mod endpoints;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(HttpAuthentication::bearer(jwt_middleware))
            .configure(endpoints::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
