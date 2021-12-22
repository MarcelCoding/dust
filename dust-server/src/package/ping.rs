use std::collections::HashMap;
use std::net::SocketAddr;
use log::{info, log};
use tokio::sync::RwLock;
use dust_networking::package::{Ping, PingPkgData, Pong, PongPkgData};
use crate::Client;

pub(super) async fn ping(
    clients: &RwLock<HashMap<SocketAddr, Client>>,
    address: &SocketAddr,
    pkg: PingPkgData,
) -> anyhow::Result<()> {
    info!("Received Ping package from {}", address.ip());

    let mut guard = clients.write().await;
    let client = guard.get_mut(address).unwrap();
    client.get_conn().lock().await.send_pkg(Pong(pkg.into())).await?;
    Ok(())
}
