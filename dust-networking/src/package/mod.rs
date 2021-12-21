use std::fs::read_to_string;

use bytes::{BufMut, BytesMut};
use log::warn;
use serde::{Deserialize, Serialize};

pub use crate::package::error::ErrorPkgData;
pub use crate::package::login::LoginPkgData;
pub use crate::package::Package::{Error, Login, Ping, Pong};
pub use crate::package::ping::PingPkgData;
pub use crate::package::pong::PongPkgData;

mod error;
mod login;
mod ping;
mod pong;

pub trait PkgData {}

pub enum Package {
    Error(ErrorPkgData),
    Ping(PingPkgData),
    Pong(PongPkgData),
    Login(LoginPkgData),
}

impl Package {
    pub(crate) fn decode(frame: &BytesMut) -> anyhow::Result<Option<Self>> {
        let id = frame[0];

        Ok(Some(match id {
            0 => Error(decode(&frame)?),
            1 => Login(decode(&frame)?),
            3 => Ping(decode(&frame)?),
            4 => Pong(decode(&frame)?),
            _ => {
                warn!("Received unknown package id {}.", id);
                return Ok(None);
            }
        }))
    }

    pub(crate) fn encode(&self) -> bincode::Result<Vec<u8>> {
        match &self {
            Error(pkg) => encode(0, pkg),
            Login(pkg) => encode(1, pkg),
            Ping(pkg) => encode(3, pkg),
            Pong(pkg) => encode(4, pkg),
        }
    }
}

fn decode<'a, T: Deserialize<'a>>(frame: &'a [u8]) -> bincode::Result<T> {
    bincode::deserialize(&frame[1..])
}

fn encode<T: ?Sized + Serialize>(id: u8, pkg: &T) -> bincode::Result<Vec<u8>> {
    let mut data = bincode::serialize(pkg)?;

    let mut result = BytesMut::with_capacity(5 + data.len());
    result.put_u32((data.len() + 1) as u32);
    result.put_u8(id);
    result.copy_from_slice(&data); // convert to move

    Ok(data)
}
