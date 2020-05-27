
#[derive(Debug)]
pub struct User {
    pub email: &'static str,
    pub username: &'static str,
}

impl User {
    pub fn new(email: &'static str, username: &'static str) -> User {
        User {
            email,
            username,
        }
    }
}

impl User {
    pub fn display_user_data(&self) {
        println!("{:?}", &self);
    }
}
