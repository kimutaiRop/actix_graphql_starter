use std::sync::Arc;

use crate::mailer::tera::Context;
use tera::{self, Tera};
mod elastic;

pub async fn send_html_email(
    to_email: &str,
    from_email: &str,
    subject: &str,
    template: &str,
    context: &Context,
    tera: Arc<Tera>,
) {
    let res = elastic::send_email(
        to_email,
        from_email,
        subject,
        template,
        context,
        tera,
    )
    .await;

    if res.is_err() {
        println!("Error sending email: {:?}", res);
    }
}
