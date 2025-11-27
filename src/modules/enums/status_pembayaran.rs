use strum_macros::EnumIter;
use serde::{ Serialize, Deserialize };
use crate::modules::enums::status_state::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Serialize, Deserialize)]
pub enum StatusPembayaran {
    Berhasil,
    Pending,
    Gagal
}

impl StatusPembayaran {
    pub fn to_state(&self) -> Box<dyn PaymentState> {
        match self {
            StatusPembayaran::Pending => Box::new(PendingState),
            StatusPembayaran::Berhasil => Box::new(BerhasilState),
            StatusPembayaran::Gagal => Box::new(GagalState),
        }
    }

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