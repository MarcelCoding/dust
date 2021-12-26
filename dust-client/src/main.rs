use std::sync::Arc;
use std::time::Duration;

use log::{error, LevelFilter};
use simplelog::{ColorChoice, ConfigBuilder, LevelPadding, TermLogger, TerminalMode};
use tokio::time;

use dust_graphics::open_window;
use dust_networking::package::{Login, LoginPkgData};

use crate::networking::Client;
use crate::package::PackageHandler;

mod networking;
mod package;
mod ping_pong;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        ConfigBuilder::new()
            .set_level_padding(LevelPadding::Right)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    open_window();

    let address = "127.0.0.1:1234".parse().unwrap();
    let pkg_handler = PackageHandler::new();
    let client = Arc::new(Client::connect(address, pkg_handler).await?);

    // let local_client = client.clone();
    // tokio::spawn(async move {
    //     loop {
    //         if let Err(err) = local_client.send_ping().await {
    //             error!("Unable to send ping: {}", err);
    //             return;
    //         } else {
    //             time::sleep(Duration::from_secs(1)).await;
    //         }
    //     }
    // });

    client.handle().await;
    Ok(())
}
