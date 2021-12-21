use std::net::SocketAddr;

use log::info;

use dust_game::user::User;
use dust_networking::package::LoginPkgData;

use crate::Client;

pub(super) fn login(client: &mut Client, address: &SocketAddr, pkg: LoginPkgData) {
    client.set_user(User::new(pkg.get_name().clone()));

    info!(
        "Client from {} logged in using name {}.",
        address,
        pkg.get_name()
    );
}
