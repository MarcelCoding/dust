use std::collections::HashMap;

use std::net::SocketAddr;

use log::warn;
use tokio::sync::RwLock;

use dust_networking::package::Package;

use crate::package::login::login;
use crate::package::ping::ping;
use crate::Client;

mod login;
mod ping;

pub struct PackageHandler {}

impl PackageHandler {
    pub fn new() -> Self {
        PackageHandler {}
    }

    pub async fn handle(
        &self,
        clients: &RwLock<HashMap<SocketAddr, RwLock<Client>>>,
        address: &SocketAddr,
        package: Package,
    ) -> anyhow::Result<()> {
        match package {
            Package::Error(_) => unimplemented(clients, address, "error").await,
            Package::Ping(pkg) => ping(clients, address, pkg).await?,
            Package::Pong(_) => unimplemented(clients, address, "pong").await,
            Package::Login(pkg) => login(clients, address, pkg).await,
        }

        Ok(())
    }
}

async fn unimplemented(
    clients: &RwLock<HashMap<SocketAddr, RwLock<Client>>>,
    address: &SocketAddr,
    pkg: &str,
) {
    let guard = clients.read().await;
    let client = guard.get(address).unwrap().read().await;

    warn!(
        "Client {} requested unimplemented package type \"{}\".",
        client.get_display(address),
        pkg
    );
}
