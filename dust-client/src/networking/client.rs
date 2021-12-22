use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::anyhow;
use futures::{stream_select, StreamExt};
use log::{error, info, warn};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use dust_networking::conn::{Connection, TcpConnection};
use dust_networking::package::Package;

use crate::package::PackageHandler;
use crate::ping_pong::PingPongHandler;

pub struct Client {
    address: SocketAddr,
    conn: Box<dyn Connection>,
    pkg_handler: PackageHandler,
    ping_pong_handler: PingPongHandler,
}

impl Client {
    pub async fn connect(address: SocketAddr, pkg_handler: PackageHandler) -> io::Result<Self> {
        let (read, write) = TcpStream::connect(address).await?.into_split();
        let conn = Box::new(TcpConnection::new(read, write).await);

        Ok(Client {
            address,
            conn,
            pkg_handler,
            ping_pong_handler: PingPongHandler::new(),
        })
    }

    pub async fn handle(&self) {
        self.on_connect();

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

    pub async fn send_ping(&mut self) -> anyhow::Result<u16> {
        self.ping_pong_handler.send_ping(&self.conn).await
    }

    fn on_connect(&self) {
        info!("Successfully established connection to {}.", self.address);
    }

    fn on_disconnect(&self, err: anyhow::Error) {
        error!("Connection to {} lost: {}", self.address, err);
    }

    async fn on_package(&self, pkg: Package) {
        match self
            .pkg_handler
            .handle(&self.conn, pkg, &self.ping_pong_handler)
            .await
        {
            Err(err) => error!(
                "Error while handling package from {}: {}",
                self.address, err
            ),
            _ => {}
        };
    }
}
