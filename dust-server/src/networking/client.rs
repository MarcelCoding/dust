use std::net::SocketAddr;
use std::sync::Arc;

use dust_game::user::User;
use dust_networking::conn::Connection;
use dust_networking::package::Package;

pub struct Client {
    conn: Arc<dyn Connection>,
    user: Option<User>,
}

impl Client {
    pub fn new(conn: Arc<dyn Connection>) -> Self {
        Client { conn, user: None }
    }

    async fn send_pkg(&self, pkg: Package) -> anyhow::Result<()> {
        self.conn.send_pkg(pkg).await
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
