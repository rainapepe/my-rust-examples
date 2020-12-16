pub struct User {
    pub name: String,
    pub age: u32,
}

pub mod model_user {

    use crate::User;

    pub fn create_user(name: String, age: u32) -> User {
        User { name, age }
    }
}
