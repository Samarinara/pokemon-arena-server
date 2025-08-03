use actix_web::{post, web, App, HttpServer, Responder};
use serde::Deserialize;
use lettre::{Message, AsyncSmtpTransport, Tokio1Executor, AsyncTransport};
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::Mailbox;
use std::collections::HashMap;
use std::error;
use lettre::message::header::ContentType;

mod serde_handler;

#[derive(Deserialize)]
struct EmailPayload {
    email: String,
}

#[post("/verify_email")]
async fn verify_email(payload: web::Json<EmailPayload>) -> impl Responder {
    let email = &payload.email;

    // Add your email verification logic here
    println!("Received email on server: {}", payload.email);
    format!("Received email: {}", email)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(verify_email))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

pub async fn send_auth_email(key: String, email: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials
    let creds_json = match serde_handler::load_json("cred.json").await {
        Ok(json) => json,
        Err(_) => {
            return Err("Missing credentials file".into());
        }
    };

    let creds: HashMap<String, String> = serde_json::from_str(&creds_json)?;
    let username = creds.get("uname").ok_or("Missing uname")?;
    let password = creds.get("pwd").ok_or("Missing pwd")?;

    // Build email
    let email_message = Message::builder()
        .from(username.parse::<Mailbox>()?)
        .to(email.parse::<Mailbox>()?)
        .subject("Pokemon Arena Verification")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Your code is: ".to_string() + &key ))?;

    // SMTP credentials and transport
    let creds = Credentials::new(username.to_string(), password.to_string());
    let mailer: AsyncSmtpTransport<Tokio1Executor> = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    // Send
    mailer.send(email_message).await?;
    Ok(())
}
