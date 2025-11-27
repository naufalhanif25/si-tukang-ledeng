use crate::modules::enums::status_pembayaran::StatusPembayaran;

#[warn(dead_code)]
pub trait PaymentState {
    fn bayar(self: Box<Self>) -> Box<dyn PaymentState>;
    fn gagal(self: Box<Self>) -> Box<dyn PaymentState>;
    fn reset(self: Box<Self>) -> Box<dyn PaymentState>;
    fn status(&self) -> StatusPembayaran;
}

pub struct PendingState;
pub struct BerhasilState;
pub struct GagalState;

impl PaymentState for PendingState {
    fn bayar(self: Box<Self>) -> Box<dyn PaymentState> {
        Box::new(BerhasilState)
    }

    fn gagal(self: Box<Self>) -> Box<dyn PaymentState> {
        Box::new(GagalState)
    }

    fn reset(self: Box<Self>) -> Box<dyn PaymentState> {
        self
    }

    fn status(&self) -> StatusPembayaran {
        StatusPembayaran::Pending
    }
}

impl PaymentState for BerhasilState {
    fn bayar(self: Box<Self>) -> Box<dyn PaymentState> {
        self
    }

    fn gagal(self: Box<Self>) -> Box<dyn PaymentState> {
        self
    }

    fn reset(self: Box<Self>) -> Box<dyn PaymentState> {
        Box::new(PendingState)
    }

    fn status(&self) -> StatusPembayaran {
        StatusPembayaran::Berhasil
    }
}

impl PaymentState for GagalState {
    fn bayar(self: Box<Self>) -> Box<dyn PaymentState> {
        Box::new(BerhasilState)
    }

    fn gagal(self: Box<Self>) -> Box<dyn PaymentState> {
        self
    }

    fn reset(self: Box<Self>) -> Box<dyn PaymentState> {
        Box::new(PendingState)
    }

    fn status(&self) -> StatusPembayaran {
        StatusPembayaran::Gagal
    }
}
