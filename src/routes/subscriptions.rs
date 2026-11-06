use std::fmt::Display;
use actix_web::{HttpResponse, Responder, web::Form};
use serde::Deserialize;


/// * We can also return with "<b>.finish()</b>"
/// 
/// * Request <b>Content-Type</b> must be <b>"application/x-www-form-urlencoded"</b> 
/// because of a <b>Form</b> type
pub async fn subscribe(subscription_form: Form<SubscriptionForm>) -> impl Responder {
    println!("Received subscription request for {}", &subscription_form.email);
    println!("Hello, {:#?}", &subscription_form.name);
    HttpResponse::Ok()
}



#[derive(Debug, Clone, Deserialize)]
pub struct SubscriptionForm {
    name: String, 
    email: Email,
}

#[derive(Debug, Clone, Deserialize)]
struct Email(String);

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}