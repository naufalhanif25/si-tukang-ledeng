use crate::modules::user::User;

#[derive(Debug, Clone, PartialEq)]
pub struct UserBuilder {
    id: String,
    pub nama: String,
    pub email: String,
    password: String,
}

impl UserBuilder {
    pub fn new(id: String) -> Self {
        Self {
            id,
            nama: String::new(),
            email: String::new(),
            password: String::new(),
        }
    }

    pub fn nama(mut self, nama: &str) -> Self {
        self.nama = nama.to_string();
        self
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = email.to_string();
        self
    }

    pub fn password(mut self, password: &str) -> Self {
        self.password = password.to_string();
        self
    }

    pub fn build(self) -> User {
        User::new(
            self.id, 
            &self.nama, 
            &self.email, 
            &self.password
        )
    }
}
