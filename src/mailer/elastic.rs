use crate::mailer::tera::Context;
use dotenvy::dotenv;
use reqwest::Client;
use std::{env, sync::Arc};
use tera::{self, Tera};

pub async fn send_email(
    to_email: &str,
    from_email: &str,
    subject: &str,
    template: &str,
    context: &Context,
    tera: Arc<Tera>,
) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("ELASTIC_API_KEY").expect("ELASTIC_API_KEY must be set");
    let client: Client = Client::new();

    let email_body = tera.render(template, context).unwrap();
    // get formatted email body
    let url = "https://api.elasticemail.com/v2/email/send";
    let mut body_text = email_body.clone();
    // remove all html tags from email body
    body_text = body_text.replace("<[^>]*>", "");

    println!("{}", body_text.as_str());

    let response = client
        .post(url)
        .form(&[
            ("apikey", api_key.as_str()),
            ("from", from_email),
            ("fromName", ""),
            ("subject", subject),
            ("to", to_email),
            ("bodyHtml", email_body.as_str()),
            ("bodyText", body_text.as_str()),
            ("isTransactional", "true"),
        ])
        .send()
        .await.unwrap();

    let status = response.status();
    if status.is_success() {
        println!("{}",response.text().await.unwrap());
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to send email",
        )))
    }
}
