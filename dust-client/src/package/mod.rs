use std::io;

use log::warn;

use dust_networking::package::Package;

pub struct PackageHandler {}

impl PackageHandler {
    pub fn new() -> Self {
        PackageHandler {}
    }

    pub async fn handle(&self, pkg: Package) -> io::Result<()> {
        match pkg {
            Package::Error(_) => unimplemented("error"),
            Package::Ping(_) => unimplemented("ping"),
            Package::Pong(_) => unimplemented("pong"),
            Package::Login(_) => unimplemented("login"),
        }

        Ok(())
    }
}

fn unimplemented(pkg: &str) {
    warn!("Server send unimplemented package type \"{}\".", pkg);
}
