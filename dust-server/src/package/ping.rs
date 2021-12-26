use std::collections::HashMap;
use std::net::SocketAddr;

use tokio::sync::RwLock;

use dust_networking::package::{PingPkgData, Pong};

use crate::Client;

pub(super) async fn ping(
    clients: &RwLock<HashMap<SocketAddr, RwLock<Client>>>,
    address: &SocketAddr,
    pkg: PingPkgData,
) -> anyhow::Result<()> {
    let guard = clients.read().await;
    let x = guard.get(address).unwrap();
    let client = x.read().await;

    // info!(
    //     "Received Ping package from {}.",
    //     client.get_display(address)
    // );
    client.send_pkg(Pong(pkg.into())).await
}
