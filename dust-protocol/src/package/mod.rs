pub use crate::package::login::LoginPackage;

mod error;
mod login;

pub trait Package {}

// pub fn from_package_id(id: u8) -> Result<dyn Package, PackageError> {
//     match id {
//         0 => Ok(E),
//         1 => Ok(LoginPackage),
//         _ => Err(PackageError::NotFound)
//     }
// }

pub enum PackageError {
    NotFound
}
