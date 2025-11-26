use crate::modules::tukang_ledeng::TukangLedeng;
use crate::modules::enums::kategori::Kategori;

#[derive(Debug, Clone, PartialEq)]
pub struct TukangLedengBuilder {
    id: String,
    pub nama: String,
    pub email: String,
    password: String,
    pub tarif: f32,
    pub kategori: Kategori,
    pub lokasi: String,
    pub rekening: i64,
    pub rekening_type: String,
}

impl TukangLedengBuilder {
    pub fn new(id: String) -> Self {
        Self {
            id,
            nama: String::new(),
            email: String::new(),
            password: String::new(),
            tarif: 0.0,
            kategori: Kategori::DomestikRumahTangga,
            lokasi: String::new(),
            rekening: 0,
            rekening_type: String::new(),
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

    pub fn tarif(mut self, tarif: f32) -> Self {
        self.tarif = tarif;
        self
    }

    pub fn kategori(mut self, kategori: Kategori) -> Self {
        self.kategori = kategori;
        self
    }

    pub fn lokasi(mut self, lokasi: &str) -> Self {
        self.lokasi = lokasi.to_string();
        self
    }

    pub fn rekening(mut self, rekening: i64) -> Self {
        self.rekening = rekening;
        self
    }

    pub fn rekening_type(mut self, rekening_type: &str) -> Self {
        self.rekening_type = rekening_type.to_string();
        self
    }

    pub fn build(self) -> TukangLedeng {
        TukangLedeng::new(
            self.id,
            &self.nama,
            &self.email,
            &self.password,
            self.tarif,
            self.kategori,
            &self.lokasi,
            self.rekening,
            self.rekening_type,
        )
    }
}
