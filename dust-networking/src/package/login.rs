use serde::{Deserialize, Serialize};

use crate::package::PkgData;

#[derive(Serialize, Deserialize)]
pub struct LoginPkgData {
    name: String,
}

impl LoginPkgData {
    pub fn new(name: String) -> Self {
        LoginPkgData { name }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl PkgData for LoginPkgData {}
