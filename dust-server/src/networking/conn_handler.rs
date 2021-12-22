use std::arch::x86_64::_rdrand32_step;
use std::collections::HashMap;
use std::io::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::anyhow;
use futures::StreamExt;
use log::{error, info, warn};
use tokio::net::tcp::WriteHalf;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use dust_networking::conn::{Connection, TcpConnection};
use dust_networking::package::Package;

use crate::{Client, PackageHandler};

pub struct ConnectionHandler {
    pkg_handler: Arc<PackageHandler>,
    clients: RwLock<HashMap<SocketAddr, RwLock<Client>>>,
}

impl ConnectionHandler {
    pub fn new(pkg_handler: Arc<PackageHandler>) -> Self {
        ConnectionHandler {
            pkg_handler,
            clients: RwLock::new(HashMap::new()),
        }
    }

    pub async fn accept(&self, mut stream: TcpStream, address: SocketAddr) {
        let (read, write) = stream.into_split();
        let connection = Arc::new(TcpConnection::new(read, write).await);

        self.on_connect(connection.clone(), &address).await;

        let err = 'connection: loop {
            let pkg = match connection.receive_pkg().await {
                Ok(Some(pkg)) => self.on_package(&address, pkg).await,
                Ok(None) => {
                    let guard = self
                        .clients
                        .read()
                        .await;
                    let client = guard
                        .get(&address)
                        .unwrap()
                        .read()
                        .await;
                    warn!(
                        "Not enough data received from {}, waiting for more.",
                        client.get_display(&address)
                    );
                    continue;
                }
                Err(err) => break 'connection anyhow!(err),
            };
        };

        self.on_disconnect(&address, err).await;
    }

    async fn on_connect(&self, conn: Arc<TcpConnection>, address: &SocketAddr) {
        let client = Client::new(conn);

        let mut clients = self.clients.write().await;
        clients.insert(address.clone(), RwLock::new(client));

        info!(
            "Client from {} established connection. Total {} clients.",
            &address,
            clients.len()
        );
    }

    async fn on_disconnect(&self, address: &SocketAddr, err: anyhow::Error) {
        let mut clients = self.clients.write().await;
        let lock = clients.remove(address).unwrap();
        let client = lock.read().await;

        info!(
            "Client from {} disconnected. Total {} clients. Reason: {}",
            client.get_display(address),
            clients.len(),
            err
        )
    }

    async fn on_package(&self, address: &SocketAddr, pkg: Package) {
        match self.pkg_handler.handle(&self.clients, address, pkg).await {
            Err(err) => error!("Error while handling package from {}: {}", &address, err),
            _ => {}
        };
    }
}
