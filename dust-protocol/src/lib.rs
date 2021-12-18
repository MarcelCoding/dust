use crate::proto::WorldMessage;

mod world;
mod boundary;
pub mod proto {
    tonic::include_proto!("dust_protocol"); // The string specified here must match the proto package name
}

fn test() {
    let w = WorldMessage{
        length: 500,
        width: 500,
        boundaries: Vec::new()
    };
}

