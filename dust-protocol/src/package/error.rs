use crate::package::Package;

pub struct ErrorPackage {
    code: ErrorCode,
}

pub enum ErrorCode {
    UnknownPackageType,
    MalformedPackage,
}

impl Package for ErrorPackage {}
