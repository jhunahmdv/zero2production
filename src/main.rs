/// To run the application, use `cargo run`
/// For the start container, database creation and migration, use `./scripts/init_db.sh`

use z2p::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let port = configuration.application_port;
    let listener =
        std::net::TcpListener::bind(&format!("127.0.0.1:{}", port)).expect("Failed to bind to address");
    run(listener)?.await
}
