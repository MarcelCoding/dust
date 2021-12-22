use serde::{Deserialize, Serialize};

use crate::package::{PingPkgData, PkgData};

#[derive(Serialize, Deserialize)]
pub struct PongPkgData {
    id: u16,
}

impl PongPkgData {
    pub fn new(id: u16) -> Self {
        PongPkgData { id }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
}

impl PkgData for PongPkgData {}

impl From<PingPkgData> for PongPkgData {
    fn from(pkg: PingPkgData) -> Self {
        PongPkgData::new(pkg.get_id())
    }
}
