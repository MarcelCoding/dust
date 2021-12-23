use async_trait::async_trait;
use bytes::{BufMut, BytesMut};
use futures::{SinkExt, StreamExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::Mutex;
use tokio_util::codec::length_delimited::Builder;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use crate::conn::Connection;
use crate::package::Package;

pub struct TcpConnection {
    framed_read: Mutex<FramedRead<OwnedReadHalf, LengthDelimitedCodec>>,
    framed_write: Mutex<FramedWrite<OwnedWriteHalf, LengthDelimitedCodec>>,
}

impl TcpConnection {
    pub async fn new(read: OwnedReadHalf, write: OwnedWriteHalf) -> Self {
        let mut length_codec_builder = Builder::new();
        length_codec_builder.max_frame_length(4096);

        let framed_read = Mutex::new(FramedRead::new(read, length_codec_builder.new_codec()));
        let framed_write = Mutex::new(FramedWrite::new(write, length_codec_builder.new_codec()));

        TcpConnection {
            framed_read,
            framed_write,
        }
    }
}

#[async_trait]
impl Connection for TcpConnection {
    async fn receive_pkg(&self) -> anyhow::Result<Option<Package>> {
        let buf = match self.framed_read.lock().await.next().await {
            None => return Ok(None),
            Some(Ok(buf)) => buf,
            Some(Err(err)) => return Err(err.into()),
        };

        Package::decode(buf)
    }

    async fn send_pkg(&self, pkg: Package) -> anyhow::Result<()> {
        let bytes = Package::encode(&pkg)?;
        let mut buf = BytesMut::with_capacity(bytes.len());

        for x in bytes {
            buf.put_u8(x);
        }

        let mut guard = self.framed_write.lock().await;
        Ok(guard.send(buf.freeze()).await?)
    }
}
