use crate::package::Package;

pub struct ErrorPackage {
    code: ErrorCode,
}

pub enum ErrorCode {
    UnknownPackageType(u8),
    MalformedPackage,
}

impl Package for ErrorPackage {}
