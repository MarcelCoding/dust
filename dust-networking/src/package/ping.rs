use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::package::PkgData;

#[derive(Serialize, Deserialize)]
pub struct PingPkgData {
    id: Uuid,
}

impl PingPkgData {
    pub fn new(id: Uuid) -> Self {
        PingPkgData { id }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
}

impl PkgData for PingPkgData {}
