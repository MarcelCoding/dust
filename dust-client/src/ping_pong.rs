use std::collections::HashMap;
use std::time::SystemTime;

use log::info;
use tokio::sync::Mutex;
use uuid::Uuid;

use dust_networking::conn::Connection;
use dust_networking::package::{Ping, PingPkgData, PongPkgData};

pub struct PingPongHandler {
    ids: Mutex<HashMap<Uuid, SystemTime>>,
}

impl PingPongHandler {
    pub fn new() -> Self {
        PingPongHandler {
            ids: Mutex::new(HashMap::new()),
        }
    }

    pub async fn send_ping(&mut self, conn: &Box<dyn Connection>) -> anyhow::Result<()> {
        let id = Uuid::new_v4();

        self.ids.lock().await.insert(id, SystemTime::now());
        conn.send_pkg(Ping(PingPkgData::new(id))).await?;

        info!("Send Ping...");

        Ok(())
    }

    pub async fn handle_pong(&self, pkg: PongPkgData) {
        let elapsed_time = self.get_elapsed_time(pkg.get_id()).await;

        info!(
            "Got back package {} after {} milliseconds.",
            pkg.get_id(),
            elapsed_time
        )
    }

    async fn get_elapsed_time(&self, id: &Uuid) -> u128 {
        return self
            .ids
            .lock()
            .await
            .remove(&id)
            .unwrap()
            .elapsed()
            .unwrap()
            .as_millis();
    }
}
