use std::io;

use dust_networking::package::Package;

pub struct PackageHandler {}

impl PackageHandler {
    pub fn new() -> Self {
        PackageHandler {}
    }

    pub async fn handle(package: Package) -> io::Result<()> {
        Ok(())
    }
}
