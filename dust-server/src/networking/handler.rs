use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use log::{info, warn};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use dust_game::user::User;

use crate::Client;
use crate::package::handle_package;

pub struct Handler {
    clients: HashMap<SocketAddr, Client>,
}

impl Handler {
    pub fn new() -> Self {
        Handler { clients: HashMap::new() }
    }

    pub async fn accept(&mut self, stream: TcpStream, address: SocketAddr) -> io::Result<()> {
        let client = self.on_connect(stream, &address);
        let mut buf: [u8; 1024] = [0; 1024];

        'connected: loop {
            match read_frame(client.get_stream_mut(), &mut buf).await {
                Ok((size, actual_read)) => {
                    if actual_read == 0 {
                        info!("Diss");
                        break 'connected; // jump to disconnect
                    }

                    let expected = size + 5;
                    if expected < actual_read {
                        warn!("Received more bytes that expected from {}. (expected: {}, actual: {})", &address, expected, actual_read);
                    } else if expected > actual_read {
                        warn!("Received not enough bytes from {}, disconnecting... (expected: {}, actual: {})", &address, expected, actual_read);
                        break 'connected;
                    }

                    let pkg_size = match actual_read < size {
                        true => actual_read,
                        false => size
                    };

                    let pkg_id = &buf[0];
                    let pkg_data = &buf[5..(pkg_size + 1)];

                    info!("Received package type {} from {}.", pkg_id, &address);
                    // self.on_package(&pkg_id, pkg_data).await;
                }
                Err(err) => {
                    warn!("Unable to read package from {}, disconnecting... ({:?})", &address, err);
                    break 'connected;
                }
            }
        }

        Ok(self.on_disconnect(&address))
    }

    fn on_connect(&mut self, stream: TcpStream, address: &SocketAddr) -> &mut Client {
        self.clients.insert(address.clone(), Client::new(stream));
        info!("Client from {} established connection. Total {} clients.", address, self.clients.len());
        self.clients.get_mut(address).unwrap()
    }

    fn on_disconnect(&mut self, address: &SocketAddr) {
        let client = self.clients.remove(address).unwrap();

        match client.get_user() {
            Some(u) => info!("Client from {} ({}) disconnected. Total {} clients.", address, u.get_name(), self.clients.len()),
            None => info!("Client from {} disconnected. Total {} clients.", address, self.clients.len())
        }
    }

    // async fn on_package(&self, id: &u8, data: &[u8]) {}
}

async fn read_frame(stream: &mut TcpStream, buf: &mut [u8]) -> anyhow::Result<(usize, usize)> {
    let actual_size = stream.read(buf).await?;
    let size: u32 = bincode::deserialize(&buf[1..5])?;
    Ok((size as usize, actual_size))
}
