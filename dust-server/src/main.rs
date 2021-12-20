use std::io;

use log::{info, LevelFilter};
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};

use crate::networking::{Client, Server};

mod networking;
mod package;

#[tokio::main]
async fn main() -> io::Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ).unwrap();

    let address = "0.0.0.0:1234".parse().unwrap();
    let server = Server::listen(&address).await?;
    info!("Server is listening on {}.", &address);

    server.handle().await
}
