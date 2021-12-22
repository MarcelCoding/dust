use std::collections::HashMap;
use std::net::SocketAddr;

use log::info;
use tokio::sync::RwLock;

use dust_game::user::User;
use dust_networking::package::LoginPkgData;

use crate::Client;

pub(super) async fn login(
    clients: &RwLock<HashMap<SocketAddr, Client>>,
    address: &SocketAddr,
    pkg: LoginPkgData,
) {
    let mut guard = clients.write().await;
    let client = guard.get_mut(address).unwrap();

    client.set_user(User::new(pkg.get_name().clone()));

    info!(
        "Client from {} logged in using name {}.",
        address,
        pkg.get_name()
    );
}
