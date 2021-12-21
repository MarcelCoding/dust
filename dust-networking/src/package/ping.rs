use serde::{Deserialize, Serialize};

use crate::package::PkgData;

#[derive(Serialize, Deserialize)]
pub struct PingPkgData {}

impl PkgData for PingPkgData {}
