use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::net::{TcpListener, TcpStream};
use tokio::time;

use crate::networking::ConnectionHandler;

pub struct Server {
    listener: TcpListener,
    conn_handler: Arc<ConnectionHandler>,
}

impl Server {
    pub async fn listen(
        address: &SocketAddr,
        conn_handler: ConnectionHandler,
    ) -> io::Result<Server> {
        let listener = TcpListener::bind(address).await?;
        Ok(Server {
            listener,
            conn_handler: Arc::new(conn_handler),
        })
    }

    // If one client fails the server is shutting down ... :D
    pub async fn handle(&self) -> io::Result<()> {
        loop {
            let (stream, address) = self.accept().await?;

            let local_handler = self.conn_handler.clone();
            tokio::spawn(async move { local_handler.accept(stream, address).await });
        }
    }

    async fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        let mut backoff = 1;

        // Try to accept a few times
        loop {
            // Perform the accept operation. If a socket is successfully
            // accepted, return it. Otherwise, save the error.
            match self.listener.accept().await {
                Ok(result) => return Ok(result),
                Err(err) => {
                    if backoff > 64 {
                        // Accept has failed too many times. Return the error.
                        return Err(err);
                    }
                }
            }

            // Pause execution until the back off period elapses.
            time::sleep(Duration::from_secs(backoff)).await;

            // Double the back off
            backoff *= 2;
        }
    }
}
