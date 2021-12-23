use dust_networking::conn::Connection;
use dust_networking::package::{PingPkgData, Pong};

pub(super) async fn ping(conn: &Box<dyn Connection>, pkg: PingPkgData) -> anyhow::Result<()> {
    conn.send_pkg(Pong(pkg.into())).await
}
