use std::collections::HashMap;
use std::io::Result;
use std::net::SocketAddr;
use log::{error, info};

use message_io::network::{Endpoint, NetEvent, Transport};
use message_io::node::{self, NodeHandler, NodeListener};

use crate::networking::Client;
use crate::package::handle_package;

pub struct Server {
    clients: HashMap<Endpoint, Client>,
    handler: NodeHandler<()>,
}

impl Server {
    pub fn listen(transport: Transport, address: SocketAddr) -> Result<(Self, NodeListener<()>)> {
        let (handler, listener) = node::split();

        match handler.network().listen(transport, address) {
            Ok(_) => Ok((Server { clients: HashMap::new(), handler }, listener)),
            Err(error) => Err(error)
        }
    }

    fn handle_accept(&mut self, endpoint: Endpoint) {
        self.clients.insert(endpoint, Client::new());
        info!("Client from {} connected. Total {} clients.", endpoint.addr(), self.clients.len());
    }

    fn handle_disconnect(&mut self, endpoint: Endpoint) {
        let client = self.clients.remove(&endpoint).unwrap();
        let user = client.get_user();

        match user {
            Some(u) => info!("Client from {} ({}) disconnected. Total {} clients.", endpoint.addr(), u.get_name(), self.clients.len()),
            None => info!("Client from {} disconnected. Total {} clients.", endpoint.addr(), self.clients.len())
        }
    }
    fn handle_message(&mut self, endpoint: Endpoint, data: &[u8]) {

        match self.clients.get_mut(&endpoint) {
            Some(client) =>
                handle_package(&endpoint, client, data[0], &data[1..])
                    .expect("Unable to parse package"),
            None => error!("Client did not connected.")
        }
    }
}

pub fn register_handler(server: &mut Server, listener: NodeListener<()>) {
    listener.for_each(move |event| match event.network() {
        NetEvent::Connected(_, _) => (),
        NetEvent::Accepted(endpoint, _) => server.handle_accept(endpoint),
        NetEvent::Message(endpoint, data) => server.handle_message(endpoint, data),
        NetEvent::Disconnected(endpoint) => server.handle_disconnect(endpoint)
    });
}
