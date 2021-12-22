use crate::Client;
use dust_networking::conn::Connection;
use dust_networking::package::{Ping, PingPkgData, Pong, PongPkgData};
use tokio::sync::Mutex;

pub(super) async fn ping(conn: &Box<dyn Connection>, pkg: PingPkgData) -> anyhow::Result<()> {
    conn.send_pkg(Pong(pkg.into())).await
}
