use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::time;

use crate::Client;
use crate::networking::Handler;

pub struct Server {
    listener: TcpListener,
    handler: Arc<Mutex<Handler>>,
}

impl Server {
    pub async fn listen(address: &SocketAddr) -> io::Result<Server> {
        let listener = TcpListener::bind(address).await?;
        Ok(Server { listener, handler: Arc::new(Mutex::new(Handler::new())) })
    }

    // If one client fails the server is shutting down ... :D
    pub async fn handle(&self) -> io::Result<()> {
        loop {
            let (stream, address) = self.accept().await?;

            let local_handler = self.handler.clone();
            tokio::spawn(async move { local_handler.lock().await.accept(stream, address).await });
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
                        return Err(err.into());
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
