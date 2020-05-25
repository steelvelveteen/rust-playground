
#[derive(Debug)]
pub struct User {
    email: &'static str,
    username: &'static str,
}

pub fn build_user(email: &'static str, username: &'static str) -> User {
    User {
        email,
        username,
    }
}
