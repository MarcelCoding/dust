use serde::{Deserialize, Serialize};

use crate::package::PkgData;

#[derive(Serialize, Deserialize)]
pub struct PongPkgData {}

impl PkgData for PongPkgData {}
