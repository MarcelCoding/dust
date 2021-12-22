use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use log::{error, info, warn};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};

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

    pub async fn accept(&self, stream: TcpStream, address: SocketAddr) {
        let connection = Mutex::new(TcpConnection::new(stream));
        let conn = Arc::new(connection);

        self.on_connect(conn.clone(), &address).await;

        let err = 'connection: loop {
            let conn = conn.clone();

            let pkg = match (conn.lock().await.receive_pkg()).await {
                Ok(pkg) => pkg,
                Err(err) => break 'connection err,
            };

            match pkg {
                Some(pkg) => self.on_package(&address, pkg).await,
                None => warn!("Not enough data, waiting for more."),
            }
        };

        self.on_disconnect(&address, err).await;
    }

    async fn on_connect(&self, conn: Arc<Mutex<dyn Connection>>, address: &SocketAddr) {
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
