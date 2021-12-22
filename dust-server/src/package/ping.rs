use std::collections::HashMap;
use std::net::SocketAddr;

use log::info;
use tokio::sync::RwLock;

use dust_networking::package::{PingPkgData, Pong};

use crate::Client;

pub(super) async fn ping(
    clients: &RwLock<HashMap<SocketAddr, RwLock<Client>>>,
    address: &SocketAddr,
    pkg: PingPkgData,
) -> anyhow::Result<()> {
    info!("Received Ping package from {}", address.ip());

    let guard = clients.read().await;
    let x = guard.get(address).unwrap();
    let client = x.write().await;
    client.send_pkg(Pong(pkg.into())).await
}
