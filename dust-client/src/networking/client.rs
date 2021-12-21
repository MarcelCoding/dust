use std::io::Result;
use std::net::SocketAddr;
use log::{error, info, warn};

use message_io::network::{Endpoint, NetEvent, Transport};
use message_io::node::{self, NodeHandler, NodeListener};

use dust_networking::package::LoginPkgData;

pub struct Client {
    handler: NodeHandler<()>,
}

impl Client {
    pub fn connect(transport: Transport, address: SocketAddr) -> Result<(Self, NodeListener<()>)> {
        let (handler, listener) = node::split();

        match handler.network().connect(transport, address) {
            Ok(_) => Ok((Client { handler }, listener)),
            Err(error) => Err(error)
        }
    }

    fn handle_connect(&self, endpoint: Endpoint, established: bool) {
        if established {
            info!("Connection to {} established.", endpoint.addr());

            let name = "Klemens";
            info!("Logging in using name {}.", name);

            let login_pkg = LoginPkgData::new(name.into());
            let mut pkg_data = bincode::serialize(&login_pkg).expect("Unable to serialize package");

            let mut data = vec![1u8];
            let i = pkg_data.len();
            let mut result = bincode::serialize(&i).unwrap();
            data.append(&mut result);
            data.append(&mut pkg_data);

            self.handler.network().send(endpoint, &data);
        } else {
            error!("Can not establish connection to {}.", endpoint.addr())
        }
    }

    fn handle_disconnect(&mut self, endpoint: Endpoint) {
        warn!("Connection to {} lost.", endpoint.addr())
    }

    fn handle_message(&self, endpoint: Endpoint, data: &[u8]) {
        info!("Server {} send fancy message.", endpoint.addr());
    }
}

pub fn register_handler(client: &mut Client, listener: NodeListener<()>) {
    listener.for_each(move |event| match event.network() {
        NetEvent::Connected(endpoint, established) => client.handle_connect(endpoint, established),
        NetEvent::Accepted(_, _) => unreachable!(),
        NetEvent::Message(endpoint, data) => client.handle_message(endpoint, data),
        NetEvent::Disconnected(endpoint) => client.handle_disconnect(endpoint)
    });
}
