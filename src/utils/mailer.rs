use crate::utils::error::ApiError;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env::var;

pub fn send(recipient: String, subject: String, body: String) -> Result<(), ApiError> {
    let email = Message::builder()
        .from(
            format!("Do Not Reply <{}>", var("MAILER_USERNAME")?)
                .parse()
                .unwrap(),
        )
        .to(recipient.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body)?;

    let creds = Credentials::new(var("MAILER_USERNAME")?, var("MAILER_PASSWORD")?);

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(ApiError::InternalServerErrorWithMessage(e.to_string())),
    }
}
