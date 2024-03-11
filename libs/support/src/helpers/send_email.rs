use error::Error;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

//forgetask_pokedex1234

pub fn send(token: &str, base_url: &str) -> Result<(), Error> {

    // Set all data needed for build message
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set in .env");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set in .env");
    let smpt_email = env::var("SMTP_EMAIL").expect("SMTP_EMAILnot set in .env");
    let email_body = format!(
        "To verify your email, follow this link:\n\n
                {}/auth/verify-email/{}",
        base_url, token
    );

    // Build the message using message builder
    let email = Message::builder()
        .from(smpt_email.parse().unwrap())
        .to(smpt_email.parse().unwrap())
        .subject("Verify your email")
        .body(email_body)
        .unwrap();

    let creds = Credentials::new(smtp_username, smtp_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::Lettre(e)),
    }
}
