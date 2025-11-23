use crate::modules::enums::metode_pembayaran::MetodePembayaran;
use crate::modules::pesanan::Pesanan;
use crate::modules::enums::kategori::Kategori;

#[derive(Debug, Clone, PartialEq)]
pub struct TukangLedeng {
    id: String,
    pub nama: String,
    pub email: String,
    password: String,
    pesanan: Vec<Pesanan>,
    pub tarif: f32,
    pub kategori: Kategori,
    pub lokasi: String,
    pub rekening: i32,
    pub rekening_type: MetodePembayaran,
}

impl TukangLedeng {
    pub fn new(id: String, nama: String, email: String, password: String, tarif: f32, kategori: Kategori, lokasi: String, rekening: i32, rekening_type: MetodePembayaran) -> Self {
        return Self {
            id,
            nama,
            email,
            password,
            pesanan: Vec::new(),
            tarif,
            kategori,
            lokasi,
            rekening,
            rekening_type
        };
    }

    pub fn set_nama(&mut self, nama: String) {
        self.nama = nama;
    }

    pub fn get_nama(&self) -> &str {
        return &self.nama;
    } 

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn get_email(&self) -> &str {
        return &self.email;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn append_pesanan(&mut self, pesanan: Pesanan) {
        self.pesanan.push(pesanan);
    }

    pub fn pop_pesanan(&mut self, id: String) {
        self.pesanan.retain(|item| *item.id != id);
    }

    pub fn get_pesanan(&self) -> &Vec<Pesanan> {
        return &self.pesanan;
    }

    pub fn set_lokasi(&mut self, lokasi: String) {
        self.lokasi = lokasi;
    }

    pub fn get_lokasi(&self) -> &str {
        return &self.lokasi;
    }

    pub fn set_tarif(&mut self, tarif: f32) {
        self.tarif = tarif;
    }

    pub fn get_tarif(&self) -> &f32 {
        return &self.tarif;
    }

    pub fn set_kategori(&mut self, kategori: Kategori) {
        self.kategori = kategori;
    }

    pub fn get_kategori(&self) -> &Kategori {
        return &self.kategori;
    }

    pub fn set_rekening(&mut self, rekening: i32) {
        self.rekening = rekening;
    }

    pub fn get_rekening(&self) -> &i32 {
        return &self.rekening;
    }

    pub fn set_rekening_type(&mut self, rekening_type: MetodePembayaran) {
        self.rekening_type = rekening_type;
    }

    pub fn get_rekening_type(&self) -> &MetodePembayaran {
        return &self.rekening_type;
    }
}