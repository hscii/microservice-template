#[cfg(test)]
mod tests {
    use actix_web::{test, App, HttpResponse, web};

    async fn ping() -> HttpResponse {
        HttpResponse::Ok().body("pong")
    }

    #[actix_web::test]
    async fn test_ping() {
        // Create the application service for testing.
        let app = test::init_service(
            App::new().route("/ping", web::get().to(ping))
        )
        .await;

        // Send a test request to the /ping endpoint.
        let req = test::TestRequest::get().uri("/ping").to_request();
        let resp = test::call_service(&app, req).await;

        // Assert the response status and body.
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "pong");
    }
}
