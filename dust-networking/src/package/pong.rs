use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::package::{PingPkgData, PkgData};

#[derive(Serialize, Deserialize)]
pub struct PongPkgData {
    id: Uuid,
}

impl PongPkgData {
    pub fn new(id: Uuid) -> Self {
        PongPkgData { id }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
}

impl PkgData for PongPkgData {}

impl From<PingPkgData> for PongPkgData {
    fn from(pkg: PingPkgData) -> Self {
        PongPkgData::new(*pkg.get_id())
    }
}
