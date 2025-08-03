use actix_web::{post, web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct EmailPayload {
    email: String,
}

#[post("/verify_email")]
async fn verify_email(payload: web::Json<EmailPayload>) -> impl Responder {
    let email = &payload.email;
    // Here you can add your email verification logic
    format!("Received email: {}", email)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(verify_email))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
