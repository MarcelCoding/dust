use std::io;

use anyhow::anyhow;
use log::{info, LevelFilter};
// use message_io::network::Transport;
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};
use dust_networking::package::{Login, LoginPkgData};

use crate::networking::Client;
use crate::package::PackageHandler;

mod networking;
mod package;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ).unwrap();

    let address = "127.0.0.1:1234".parse().unwrap();
    let pkg_handler = PackageHandler::new();
    let mut client = Client::connect(address, pkg_handler).await?;
    client.send_pkg(Login(LoginPkgData::new("Marcel Davis".to_string()))).await?;

    client.handle().await;
    Ok(())
}
