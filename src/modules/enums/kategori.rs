use strum_macros::EnumIter;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Serialize, Deserialize)]
pub enum Kategori {
    DomestikRumahTangga,
    Komersial,
    Industri,
    ProyekKonstruksiBaru,
    Renovasi,
    PerawatanServisRutin,
    Darurat,
    FasilitasPublik,
    GedungTinggi,
    KapalMaritim
}

impl Kategori {
    pub fn as_string(&self) -> &'static str {
        match self {
            Kategori::DomestikRumahTangga => "Domestik Rumah Tangga",
            Kategori::Komersial => "Komersial",
            Kategori::Industri => "Industri",
            Kategori::ProyekKonstruksiBaru => "Proyek Konstruksi Baru",
            Kategori::Renovasi => "Renovasi",
            Kategori::PerawatanServisRutin => "Perawatan / Servis Rutin",
            Kategori::Darurat => "Darurat",
            Kategori::FasilitasPublik => "Fasilitas Publik",
            Kategori::GedungTinggi => "Gedung Tinggi",
            Kategori::KapalMaritim => "Kapal Maritim",
        }
    }

    pub fn from_string(input: &str) -> Option<Kategori> {
        match input {
            "Domestik Rumah Tangga" => Some(Kategori::DomestikRumahTangga),
            "Komersial" => Some(Kategori::Komersial),
            "Industri" => Some(Kategori::Industri),
            "Proyek Konstruksi Baru" => Some(Kategori::ProyekKonstruksiBaru),
            "Renovasi" => Some(Kategori::Renovasi),
            "Perawatan / Servis Rutin" => Some(Kategori::PerawatanServisRutin),
            "Darurat" => Some(Kategori::Darurat),
            "Fasilitas Publik" => Some(Kategori::FasilitasPublik),
            "Gedung Tinggi" => Some(Kategori::GedungTinggi),
            "Kapal Maritim" => Some(Kategori::KapalMaritim),
            _ => None,
        }
    }
}
