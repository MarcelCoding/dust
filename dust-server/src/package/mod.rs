use std::io;
use std::net::SocketAddr;

use log::warn;

use dust_networking::package::Package;

use crate::package::login::login;
use crate::Client;

mod login;

pub struct PackageHandler {}

impl PackageHandler {
    pub fn new() -> Self {
        PackageHandler {}
    }

    pub async fn handle(
        &self,
        client: &mut Client,
        address: &SocketAddr,
        package: Package,
    ) -> io::Result<()> {
        match package {
            Package::Error(_) => unimplemented(client, address, "error"),
            Package::Ping(_) => unimplemented(client, address, "ping"),
            Package::Pong(_) => unimplemented(client, address, "pong"),
            Package::Login(pkg) => login(client, address, pkg),
        }

        Ok(())
    }
}

fn unimplemented(client: &Client, address: &SocketAddr, pkg: &str) {
    warn!(
        "Client {} requested unimplemented package type \"{}\".",
        client.get_display(address),
        pkg
    );
}
