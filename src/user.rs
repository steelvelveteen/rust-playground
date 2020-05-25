
#[derive(Debug)]
pub struct User {
    pub email: &'static str,
    pub username: &'static str,
}

pub fn build_user(email: &'static str, username: &'static str) -> User {
    User {
        email,
        username,
    }
}
