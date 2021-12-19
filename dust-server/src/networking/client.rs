use dust_game::user::User;

pub struct Client {
    user: Option<User>,
}

impl Client {
    pub fn new() -> Self {
        Client { user: None }
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
