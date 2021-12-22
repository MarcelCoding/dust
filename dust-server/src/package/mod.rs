use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;

use log::warn;
use tokio::sync::RwLock;

use dust_networking::package::Package;

use crate::Client;
use crate::package::login::login;

mod login;

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
    ) -> io::Result<()> {
        match package {
            Package::Error(_) => unimplemented(clients, address, "error").await,
            Package::Ping(_) => unimplemented(clients, address, "ping").await,
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
