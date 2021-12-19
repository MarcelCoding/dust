use bincode::deserialize;
use log::{error, info};
use message_io::network::Endpoint;

use dust_game::user::User;
use dust_protocol::package::LoginPackage;

use crate::networking::Client;

pub fn handle_package(endpoint: &Endpoint, client: &mut Client, pkg_id: u8, pkg_data: &[u8]) -> bincode::Result<()> {
    match pkg_id {
        1 => {
            match client.get_user() {
                Some(user) => error!("Client {} is already registered as {}.", endpoint.addr(), user.get_name()),
                None => {
                    let login_pkg: LoginPackage = deserialize(pkg_data)?;
                    client.set_user(User::new(login_pkg.get_name().clone()));

                    info!("Client from {} logged in using name {}.", endpoint.addr(), login_pkg.get_name());
                }
            }
        }
        _ => error!("Client {} send unknown package with id {}.", endpoint.addr(), pkg_id) // TODO: send feedback to client
    }
    Ok(())
}
