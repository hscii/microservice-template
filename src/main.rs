use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use supabase_actix_auth_middleware::jwt_middleware;
use template::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(jwt_middleware);
        App::new().wrap(auth).configure(config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
