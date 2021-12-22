use serde::{Deserialize, Serialize};

use crate::package::PkgData;

#[derive(Serialize, Deserialize)]
pub struct PingPkgData {
    id: u16,
}

impl PingPkgData {
    pub fn new(id: u16) -> Self {
        PingPkgData { id }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
}

impl PkgData for PingPkgData {}
