use strum_macros::EnumIter;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Serialize, Deserialize)]
pub enum StatusPembayaran {
    Berhasil,
    Pending,
    Gagal
}

impl StatusPembayaran {
    pub fn as_string(&self) -> &'static str {
        match self {
            StatusPembayaran::Berhasil => "Berhasil",
            StatusPembayaran::Pending => "Pending",
            StatusPembayaran::Gagal => "Gagal",
        }
    }

    pub fn from_string(input: &str) -> StatusPembayaran {
        match input {
            "Berhasil" => StatusPembayaran::Berhasil,
            "Pending" => StatusPembayaran::Pending,
            "Gagal" => StatusPembayaran::Gagal,
            _ => StatusPembayaran::Pending,
        }
    }
}