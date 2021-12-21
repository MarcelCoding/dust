use std::io;
use std::net::SocketAddr;

use log::{error, info, warn};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use dust_networking::conn::{Connection, TcpConnection};
use dust_networking::package::Package;

use crate::package::PackageHandler;

pub struct Client {
    address: SocketAddr,
    conn: Mutex<Box<dyn Connection>>,
    pkg_handler: PackageHandler,
}

impl Client {
    pub async fn connect(address: SocketAddr, pkg_handler: PackageHandler) -> io::Result<Self> {
        let conn = Box::new(TcpConnection::new(TcpStream::connect(address).await?));
        Ok(Client {
            address,
            conn: Mutex::new(conn),
            pkg_handler,
        })
    }

    pub async fn handle(&mut self) {
        self.on_connect();

        let err = 'connection: loop {
            let pkg = match (&mut self.conn.lock().await.receive_pkg()).await {
                Ok(pkg) => pkg,
                Err(err) => break 'connection err,
            };

            match pkg {
                Some(pkg) => self.on_package(pkg).await,
                None => warn!("Not enough data, waiting for more."),
            }
        };

        self.on_disconnect(err);
    }

    pub async fn send_pkg(&mut self, pkg: Package) -> anyhow::Result<()> {
        self.conn.lock().await.send_pkg(pkg).await
    }

    // pub async fn send(&mut self, package: &T) {
    //     let mut pkg_data = bincode::serialize(package).expect("Unable to serialize Package");
    //     let pkg_data_length = pkg_data.len();
    //     let mut serialized_length = bincode::serialize(&pkg_data_length).unwrap();
    //
    //     let mut to_send = vec![1u8];
    //     to_send.append(&mut serialized_length);
    //     to_send.append(&mut pkg_data);
    //
    //     self.socket.write_all(&to_send).await?;
    // }

    // async fn handle_connect(&mut self) {
    //     let name = "Marcel Davis";
    //     let login_pkg = LoginPackage::new(name.into());
    //
    //     self.send(&login_pkg);
    // }

    fn on_connect(&self) {
        info!("Successfully established connection to {}.", self.address,);
    }

    fn on_disconnect(&self, err: anyhow::Error) {
        error!("Connection to {} lost: {}", self.address, err,);
    }

    async fn on_package(&self, pkg: Package) {
        match self.pkg_handler.handle(pkg).await {
            Err(err) => error!(
                "Error while handling package from {}: {}",
                self.address, err
            ),
            _ => {}
        };
    }
}
