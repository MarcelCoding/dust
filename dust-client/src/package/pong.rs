use tokio::sync::RwLock;

use dust_networking::package::PongPkgData;

use crate::ping_pong::PingPongHandler;

pub(super) async fn pong(
    pkg: PongPkgData,
    ping_pong_handler: &RwLock<PingPongHandler>,
) -> anyhow::Result<()> {
    ping_pong_handler.write().await.handle_pong(pkg).await;
    Ok(())
}
