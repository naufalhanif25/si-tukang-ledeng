use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum StatusPembayaran {
    Berhasil,
    Pending,
    Gagal,
}

impl StatusPembayaran {
    pub fn as_string(&self) -> &'static str {
        match self {
            StatusPembayaran::Berhasil => "Berhasil",
            StatusPembayaran::Pending => "Pending",
            StatusPembayaran::Gagal => "Gagal",
        }
    }

    pub fn from_string(input: &str) -> Option<StatusPembayaran> {
        match input {
            "Berhasil" => Some(StatusPembayaran::Berhasil),
            "Pending" => Some(StatusPembayaran::Pending),
            "Gagal" => Some(StatusPembayaran::Gagal),
            _ => None,
        }
    }
}