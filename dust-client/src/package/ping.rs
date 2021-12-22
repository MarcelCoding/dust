use tokio::sync::Mutex;
use dust_networking::conn::Connection;
use dust_networking::package::{Ping, PingPkgData, Pong, PongPkgData};
use crate::Client;

pub(super) async fn ping(conn: &Mutex<Box<dyn Connection>>, pkg: PingPkgData) -> anyhow::Result<()> {
    conn.lock().await.send_pkg(Pong(pkg.into())).await
}
