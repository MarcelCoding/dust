use log::{info, warn};
use tokio::sync::RwLock;

use dust_networking::conn::Connection;
use dust_networking::package::Package;

use crate::package::ping::ping;
use crate::package::pong::pong;
use crate::ping_pong::PingPongHandler;

mod ping;
mod pong;

pub struct PackageHandler {
    ping_pong_handler: RwLock<PingPongHandler>,
}

impl PackageHandler {
    pub fn new() -> Self {
        PackageHandler {
            ping_pong_handler: RwLock::new(PingPongHandler::new()),
        }
    }

    pub async fn handle(&self, conn: &Box<dyn Connection>, pkg: Package) -> anyhow::Result<()> {
        match pkg {
            Package::Error(_) => unimplemented("error"),
            Package::Ping(pkg) => ping(conn, pkg).await?,
            Package::Pong(pkg) => pong(pkg, &self.ping_pong_handler).await?,
            Package::Login(_) => unimplemented("login"),
        }
        // info!("Received Package");

        Ok(())
    }

    pub async fn send_ping(&self, conn: &Box<dyn Connection>) -> anyhow::Result<()> {
        self.ping_pong_handler.write().await.send_ping(conn).await
    }
}

fn unimplemented(pkg: &str) {
    warn!("Server send unimplemented package type \"{}\".", pkg);
}
