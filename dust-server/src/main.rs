use std::io;
use std::sync::Arc;

use log::{info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::networking::{Client, ConnectionHandler, Server};
use crate::package::PackageHandler;

mod networking;
mod package;

#[tokio::main]
async fn main() -> io::Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let package_handler = Arc::new(PackageHandler::new());
    let conn_handler = ConnectionHandler::new(package_handler.clone());

    let address = "0.0.0.0:1234".parse().unwrap();
    let server = Server::listen(&address, conn_handler).await?;
    info!("Server is listening on {}.", &address);

    server.handle().await
}
