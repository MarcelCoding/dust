use serde::{Deserialize, Serialize};

use crate::package::PkgData;

#[derive(Serialize, Deserialize)]
pub struct ErrorPkgData {
    code: ErrorCode,
}

#[derive(Serialize, Deserialize)]
pub enum ErrorCode {
    UnknownPackageType(u8),
    MalformedPackage,
}

impl PkgData for ErrorPkgData {}
