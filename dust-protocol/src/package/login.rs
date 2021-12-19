use serde::{Deserialize, Serialize};

use crate::package::Package;

#[derive(Serialize, Deserialize)]
pub struct LoginPackage {
    name: String,
}

impl LoginPackage {
    pub fn new(name: String) -> Self {
        LoginPackage { name }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl Package for LoginPackage {}
