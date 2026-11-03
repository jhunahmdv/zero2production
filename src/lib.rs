use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server, web};

pub fn run() -> Result<Server, std::io::Error> {
    let app = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health", web::get().to(health))
    })
    .bind("127.0.0.1:8000")?
    .run();
    // .await
    Ok(app)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

/// We can also return with "<b>.finish()</b>"
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
