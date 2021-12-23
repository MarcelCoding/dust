use std::collections::HashMap;
use std::time::SystemTime;

use log::info;
use rand::random;
use tokio::sync::Mutex;

use dust_networking::conn::Connection;
use dust_networking::package::{Ping, PingPkgData, PongPkgData};

pub struct PingPongHandler {
    ids: Mutex<HashMap<u16, SystemTime>>,
}

impl PingPongHandler {
    pub fn new() -> Self {
        PingPongHandler {
            ids: Mutex::new(HashMap::new()),
        }
    }

    pub async fn send_ping(&mut self, conn: &Box<dyn Connection>) -> anyhow::Result<u16> {
        let mut ids_guard = self.ids.lock().await;

        let id: u16 = 'id: loop {
            let id: u16 = random();
            if !ids_guard.contains_key(&id) {
                break 'id id;
            }
        };

        ids_guard.insert(id, SystemTime::now());
        conn.send_pkg(Ping(PingPkgData::new(id))).await?;

        info!("Send Ping...");

        Ok(id)
    }

    pub async fn handle_pong(&self, pkg: PongPkgData) {
        let mut ids_guard = self.ids.lock().await;

        let pkg_id = pkg.get_id();
        let time = ids_guard.remove(&pkg_id).unwrap();
        let elapsed_time = time.elapsed().unwrap().as_millis();

        info!(
            "Got back package {} after {} milliseconds.",
            pkg_id, elapsed_time
        )
    }
}
