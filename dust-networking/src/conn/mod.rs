use async_trait::async_trait;

pub use crate::conn::tcp::TcpConnection;
use crate::package::Package;

mod tcp;

#[async_trait]
pub trait Connection: Send + Sync {
    async fn receive_pkg(&self) -> anyhow::Result<Option<Package>>;
    async fn send_pkg(&self, pkg: Package) -> anyhow::Result<()>;
}
