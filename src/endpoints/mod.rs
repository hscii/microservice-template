// Aggregates all endpoints into a single module
pub mod ping;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(ping::ping);
    // Add future endpoints here
}
