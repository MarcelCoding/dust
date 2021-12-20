use tokio::net::TcpStream;

use dust_game::user::User;

pub struct Client {
    stream: TcpStream,
    user: Option<User>,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Client { stream, user: None }
    }

    pub fn get_stream(&self) -> &TcpStream {
        &self.stream
    }

    pub fn get_stream_mut(&mut self) -> &mut TcpStream {
        &mut self.stream
    }

    pub fn get_user(&self) -> &Option<User> {
        &self.user
    }

    pub fn set_user(&mut self, user: User) {
        self.user = Some(user);
    }

    pub fn is_logged_in(&self) -> bool {
        self.user.is_some()
    }
}
