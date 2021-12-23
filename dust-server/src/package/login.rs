use std::collections::HashMap;
use std::net::SocketAddr;

use log::info;
use tokio::sync::RwLock;

use dust_game::user::User;
use dust_networking::package::LoginPkgData;

use crate::Client;

pub(super) async fn login(
    clients: &RwLock<HashMap<SocketAddr, RwLock<Client>>>,
    address: &SocketAddr,
    pkg: LoginPkgData,
) {
    let mut guard = clients.write().await;
    let mut client = guard.get_mut(address).unwrap().write().await;

    client.set_user(User::new(pkg.get_name().clone()));

    info!(
        "Client from {} logged in using name {}.",
        address,
        pkg.get_name()
    );
}
