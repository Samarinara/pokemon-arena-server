use actix_web::{post, web, App, HttpServer, Responder};
use serde::Deserialize;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, SmtpTransport, Tokio1Executor};
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::Mailbox;
use std::collections::HashMap;
use std::error;
use lettre::message::header::ContentType;

mod serde_handler;

#[derive(Deserialize)]
struct EmailPayload {
    email: String,
    key: String,
}

#[post("/verify_email")]
async fn verify_email(payload: web::Json<EmailPayload>) -> impl Responder {
    let email = &payload.email;
    let key = &payload.key;

    // Add your email verification logic here
    println!("Received email on server: {0}, key: {1}", &email, &key);
    send_auth_email(key.to_string(), email).unwrap();
    "Email sent!".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(verify_email))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
pub fn send_auth_email(key: String, email: &str,) -> Result<(), Box<dyn std::error::Error>> {

    // Load credentials synchronously
    let creds_json = serde_handler::load_json("cred.json")?;
    let creds: HashMap<String, String> = serde_json::from_str(&creds_json)?;
    let username = creds.get("uname").ok_or("Missing uname")?;
    let password = creds.get("pwd").ok_or("Missing pwd")?;
 
    // Build email
    let email_message = Message::builder()
        .from(username.parse::<Mailbox>()?)
        .to(email.parse::<Mailbox>()?)
        .subject("Pokemon Arena Verification")
        .header(ContentType::TEXT_PLAIN)
        .body(format!("Your code is: {}", key))?;

    // SMTP credentials and transport
    let creds = Credentials::new(username.to_string(), password.to_string());

    // Use blocking SMTP transport instead of async
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    // Send email synchronously
    lettre::Transport::send(&mailer, &email_message)?;

    Ok(())
}
