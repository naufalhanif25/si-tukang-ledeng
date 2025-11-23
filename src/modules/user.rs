use crate::modules::pesanan::Pesanan;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    id: String,
    pub nama: String,
    pub email: String,
    password: String,
    keranjang: Vec<Pesanan>,
}

impl User {
    pub fn new(id: String, nama: String, email: String, password: String) -> Self {
        return Self { 
            id, 
            nama, 
            email, 
            password, 
            keranjang: Vec::new() 
        };
    }

    pub fn set_nama(&mut self, nama: String) {
        self.nama = nama;
    }

    pub fn get_nama(&self) -> &str {
        return &self.nama
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn append_keranjang(&mut self, pesanan: Pesanan) {
        self.keranjang.push(pesanan);
    }

    pub fn pop_keranjang(&mut self, id: String) {
        self.keranjang.retain(|item| *item.id != id);
    }

    pub fn get_keranjang(&self) -> &Vec<Pesanan> {
        &self.keranjang
    }
}
