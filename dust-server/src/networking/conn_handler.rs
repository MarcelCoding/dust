use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use log::info;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use dust_networking::conn::TcpConnection;
use dust_networking::package::Package;

use crate::{Client, PackageHandler};

pub struct ConnectionHandler {
    pkg_handler: Arc<PackageHandler>,
    clients: Mutex<HashMap<SocketAddr, Client>>,
}

impl ConnectionHandler {
    pub fn new(pkg_handler: Arc<PackageHandler>) -> Self {
        ConnectionHandler {
            pkg_handler,
            clients: Mutex::new(HashMap::new()),
        }
    }

    pub async fn accept(&mut self, stream: TcpStream, address: SocketAddr) {
        self.on_connect(stream, &address).await;

        let mut clients = self.clients.lock().await;
        let client = clients.get_mut(&address).unwrap();
        let conn = client.get_conn();

        let err = 'connection: loop {
            match (&mut conn.receive_pkg()).await {
                Ok(Some(pkg)) => self.on_package(pkg).await,
                Ok(None) => info!("Not enough data, waiting for more."),
                Err(err) => break 'connection err,
            }
        };

        self.on_disconnect(&address, err).await;
    }

    async fn on_connect(&self, stream: TcpStream, address: &SocketAddr) {
        let conn = Box::new(TcpConnection::new(stream));
        let client = Client::new(conn);

        let mut clients = self.clients.lock().await;
        clients.insert(address.clone(), client);

        info!(
            "Client from {} established connection. Total {} clients.",
            &address,
            clients.len()
        );
    }

    async fn on_disconnect(&self, address: &SocketAddr, err: anyhow::Error) {
        let mut clients = self.clients.lock().await;
        let client = clients.remove(address).unwrap();

        match client.get_user() {
            Some(u) => info!(
                "Client from {} ({}) disconnected. Total {} clients. Reason: {}",
                address,
                u.get_name(),
                clients.len(),
                err
            ),
            None => info!(
                "Client from {} disconnected. Total {} clients. Reason: {}",
                address,
                clients.len(),
                err
            ),
        }
    }

    async fn on_package(&self, pkg: Package) {}
}
