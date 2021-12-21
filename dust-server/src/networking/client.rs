use std::net::SocketAddr;

use dust_game::user::User;
use dust_networking::conn::Connection;

pub struct Client {
    conn: Box<dyn Connection>,
    user: Option<User>,
}

impl Client {
    pub fn new(conn: Box<dyn Connection>) -> Self {
        Client { conn, user: None }
    }

    pub fn get_conn(&mut self) -> &mut Box<dyn Connection> {
        &mut self.conn
    }

    pub fn get_user(&self) -> &Option<User> {
        &self.user
    }

    pub fn set_user(&mut self, user: User) {
        self.user = Some(user);
    }

    pub fn get_display(&self, address: &SocketAddr) -> String {
        match &self.user {
            None => address.to_string(),
            Some(user) => format!("{} ({})", address.to_string(), user.get_name()),
        }
    }
}
