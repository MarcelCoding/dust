use crate::Client;
use dust_networking::package::{Ping, PingPkgData, Pong, PongPkgData};
use log::{info, log};
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::RwLock;

pub(super) async fn ping(
    clients: &RwLock<HashMap<SocketAddr, RwLock<Client>>>,
    address: &SocketAddr,
    pkg: PingPkgData,
) -> anyhow::Result<()> {
    info!("Received Ping package from {}", address.ip());

    let mut guard = clients.read().await;
    let x = guard.get(address).unwrap();
    let client = x.write().await;
    // TODO:
    // client.get_conn().lock().await.send_pkg(Pong(pkg.into())).await?;
    Ok(())
}
