use strum_macros::EnumIter;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Serialize, Deserialize)]
pub enum MetodePembayaran {
    TfBsi,
    TfBca,
    TfBri,
    TfBankAceh,
    EWalleDana,
    EWalletOvo,
    EWalletGopay,
    EWalletLinkaja
}

impl MetodePembayaran {
    pub fn as_string(&self) -> &'static str {
        match self {
            MetodePembayaran::TfBsi => "Transfer BSI",
            MetodePembayaran::TfBca => "Transfer BCA",
            MetodePembayaran::TfBri => "Transfer BRI",
            MetodePembayaran::TfBankAceh => "Transfer Bank Aceh",
            MetodePembayaran::EWalleDana => "E-Wallet Dana",
            MetodePembayaran::EWalletOvo => "E-Wallet OVO",
            MetodePembayaran::EWalletGopay => "E-Wallet GoPay",
            MetodePembayaran::EWalletLinkaja => "E-Wallet LinkAja",
        }
    }

    pub fn from_string(input: &str) -> Option<MetodePembayaran> {
        match input {
            "Transfer BSI" => Some(MetodePembayaran::TfBsi),
            "Transfer BCA" => Some(MetodePembayaran::TfBca),
            "Transfer BRI" => Some(MetodePembayaran::TfBri),
            "Transfer Bank Aceh" => Some(MetodePembayaran::TfBankAceh),
            "E-Wallet Dana" => Some(MetodePembayaran::EWalleDana),
            "E-Wallet OVO" => Some(MetodePembayaran::EWalletOvo),
            "E-Wallet GoPay" => Some(MetodePembayaran::EWalletGopay),
            "E-Wallet LinkAja" => Some(MetodePembayaran::EWalletLinkaja),
            _ => None,
        }
    }
}