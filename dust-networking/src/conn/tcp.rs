use async_trait::async_trait;
use bytes::{Buf, BytesMut};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use crate::conn::Connection;
use crate::package::Package;

pub struct TcpConnection {
    stream: TcpStream,
    buffer: BytesMut,
    pkg_size: Option<usize>,
}

impl TcpConnection {
    pub fn new(stream: TcpStream) -> Self {
        TcpConnection {
            stream,
            buffer: BytesMut::with_capacity(4096),
            pkg_size: None,
        }
    }

    fn parse_pkg(&mut self) -> anyhow::Result<Option<Package>> {
        // read package size if last package was finished
        if self.pkg_size == None {
            // package size (u32) consists of 4 bytes
            if self.buffer.remaining() < 4 {
                return Ok(None);
            }

            let pkg_size = &self.buffer.get_u32();
            self.pkg_size = Some(pkg_size.clone() as usize);
        }

        let pkg_size = self.pkg_size.unwrap();

        // check if the buf is a full package
        if self.buffer.remaining() < pkg_size {
            return Ok(None);
        }

        // trying to decode package
        match Package::decode(&self.buffer)? {
            Some(pkg) => {
                // discard bytes from current package
                self.buffer.advance(pkg_size);
                Ok(Some(pkg))
            }
            None => Ok(None),
        }
    }
}

#[async_trait]
impl Connection for TcpConnection {
    async fn receive_pkg(&mut self) -> anyhow::Result<Option<Package>> {
        loop {
            if let Some(frame) = self.parse_pkg()? {
                return Ok(Some(frame));
            }

            if 0 == self.stream.read(&mut self.buffer).await? {
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    //TODO:
                    panic!("Connection reset by peer")
                    // Err("connection reset by peer".into())
                };
            }
        }
    }

    async fn send_pkg(&self, pkg: Package) -> anyhow::Result<()> {
        todo!()
    }
}
