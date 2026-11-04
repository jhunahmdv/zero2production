use std::{fmt::Display, net::TcpListener};
use serde::Deserialize;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server, web::{self, Form}};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let app = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health", web::get().to(health))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    // .await
    Ok(app)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// * We can also return with "<b>.finish()</b>"
/// 
/// * Request <b>Content-Type</b> must be <b>"application/x-www-form-urlencoded"</b> 
/// because of a <b>Form</b> type
async fn subscribe(subscription_form: Form<SubscriptionForm>) -> impl Responder {
    println!("Received subscription request for {}", &subscription_form.email);
    println!("Hello, {:#?}", &subscription_form.name);
    HttpResponse::Ok()
}

#[derive(Debug, Clone, Deserialize)]
struct SubscriptionForm {
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