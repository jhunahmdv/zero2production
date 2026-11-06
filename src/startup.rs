use actix_web::{App, HttpServer, dev::Server, web};
use std::net::TcpListener;

use crate::routes::{greet, health, subscribe};

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