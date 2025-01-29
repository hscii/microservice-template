use actix_web::{App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use log::{debug, info};
use std::env;

pub mod endpoints;
pub mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env if available
    dotenv().ok();

    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Starting server...");
    debug!("Log level set to 'debug'");

    // Get the bind address from environment variable or use default (127.0.0.1:8080)
    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    info!("Binding server to {}", bind_address);

    HttpServer::new(move || App::new().configure(endpoints::configure_routes))
        .bind(bind_address)?
        .run()
        .await
}
