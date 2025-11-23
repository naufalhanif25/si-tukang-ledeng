#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetodePembayaran {
    TfBsi,
    TfBca,
    TfBri,
    TfBankAceh,
    EWalleDana,
    EWalletOvo,
    EWalletGopay,
    EWalletLinkaja,
}
