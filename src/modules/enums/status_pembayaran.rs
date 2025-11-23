#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusPembayaran {
    Berhasil,
    Pending,
    Gagal,
}