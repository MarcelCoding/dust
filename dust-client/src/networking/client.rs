use std::io;
use std::net::SocketAddr;

use anyhow::anyhow;
use log::{error, info, warn};
use tokio::net::TcpStream;

use dust_networking::conn::{Connection, TcpConnection};
use dust_networking::package::Package;

use crate::package::PackageHandler;

pub struct Client {
    address: SocketAddr,
    conn: Box<dyn Connection>,
    pkg_handler: PackageHandler,
}

impl Client {
    pub async fn connect(address: SocketAddr, pkg_handler: PackageHandler) -> io::Result<Self> {
        let (read, write) = TcpStream::connect(address).await?.into_split();
        let conn = Box::new(TcpConnection::new(read, write).await);

        let client = Client {
            address,
            conn,
            pkg_handler,
        };

        client.on_connect();

        Ok(client)
    }

    pub async fn handle(&self) {
        info!("Starting handling of incoming packages.");

        let err = 'connection: loop {
            let pkg = match self.conn.receive_pkg().await {
                Ok(Some(pkg)) => pkg,
                Err(err) => break 'connection anyhow!(err),
                Ok(None) => {
                    warn!("Not enough data received waiting for more.",);
                    continue;
                } // todo: add client display name
            };

            self.on_package(pkg).await;
        };

        self.on_disconnect(err);
    }

    pub async fn send_pkg(&self, pkg: Package) -> anyhow::Result<()> {
        self.conn.send_pkg(pkg).await
    }

    pub async fn send_ping(&self) -> anyhow::Result<u16> {
        self.pkg_handler.send_ping(&self.conn).await
    }

    fn on_connect(&self) {
        info!("Successfully established connection to {}.", self.address);
    }

    fn on_disconnect(&self, err: anyhow::Error) {
        error!("Connection to {} lost: {}", self.address, err);
    }

    async fn on_package(&self, pkg: Package) {
        if let Err(err) = self.pkg_handler.handle(&self.conn, pkg).await {
            error!(
                "Error while handling package from {}: {}",
                self.address, err
            )
        };
    }
}
