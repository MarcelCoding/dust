use log::LevelFilter;
use simplelog::{ColorChoice, ConfigBuilder, LevelPadding, TerminalMode, TermLogger};

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
    )
        .unwrap();

    let address = "192.168.2.219:1234".parse().unwrap();
    let pkg_handler = PackageHandler::new();
    let mut client = Client::connect(address, pkg_handler).await?;

    client
        .send_pkg(Login(LoginPkgData::new("Tilman? More like Tilfrau".to_string())))
        .await?;

    client.send_ping().await?;

    client.handle().await;
    Ok(())
}
