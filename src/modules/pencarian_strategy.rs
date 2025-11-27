use crate::modules::tukang_ledeng::TukangLedeng;
use crate::modules::enums::kategori::Kategori;

pub trait PencarianStrategy {
    fn run<'a>(&self, sumber: &'a Vec<TukangLedeng>) -> Vec<&'a TukangLedeng>;
}

pub struct CariStrategy { pub kata_kunci: String }
pub struct FilterStrategy { pub kategori: Kategori }

impl PencarianStrategy for CariStrategy {
    fn run<'a>(&self, sumber: &'a Vec<TukangLedeng>) -> Vec<&'a TukangLedeng> {
        sumber.iter().filter(|item| item.nama.contains(&self.kata_kunci) || item.lokasi.contains(&self.kata_kunci)).collect()
    }
}

impl PencarianStrategy for FilterStrategy {
    fn run<'a>(&self, sumber: &'a Vec<TukangLedeng>) -> Vec<&'a TukangLedeng> {
        sumber.iter().filter(|item| *item.get_kategori() == self.kategori).collect()
    }
}
