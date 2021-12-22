use std::io;

use log::{info, warn};
use tokio::sync::Mutex;
use dust_networking::conn::Connection;

use dust_networking::package::Package;

mod ping;

use crate::ping_pong::PingPongHandler;
use crate::package::ping::ping;

pub struct PackageHandler {}

impl PackageHandler {
    pub fn new() -> Self {
        PackageHandler {}
    }

    pub async fn handle(&self, conn: &Mutex<Box<dyn Connection>>, pkg: Package, ping_pong_handler: &PingPongHandler) -> anyhow::Result<()> {
        match pkg {
            Package::Error(_) => unimplemented("error"),
            Package::Ping(pkg) => ping(conn, pkg).await?,
            Package::Pong(pkg) => ping_pong_handler.handle_pong(pkg).await,
            Package::Login(_) => unimplemented("login"),
        }
        info!("Received Package");

        Ok(())
    }
}

fn unimplemented(pkg: &str) {
    warn!("Server send unimplemented package type \"{}\".", pkg);
}
