use crate::modules::tukang_ledeng::TukangLedeng;
use crate::modules::enums::kategori::Kategori;
use crate::modules::pencarian_strategy::PencarianStrategy;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CariTukangLedeng<'a> { 
    sumber: &'a Vec<TukangLedeng> 
}

impl<'a> CariTukangLedeng<'a> {
    pub fn new(sumber: &'a Vec<TukangLedeng>) -> Self { 
        Self { sumber } 
    }

    pub fn run_strategy(&self, strategy: &dyn PencarianStrategy) -> Vec<&TukangLedeng> {
        strategy.run(self.sumber)
    }

    pub fn cari(&self, kata_kunci: &str) -> Vec<&TukangLedeng> { 
        self.sumber.iter().filter(|item| item.nama.contains(kata_kunci) || item.lokasi.contains(kata_kunci)).collect()
    }

    pub fn filter(&self, kategori: Kategori) -> Vec<&TukangLedeng> { 
        self.sumber.iter().filter(|item| *item.get_kategori() == kategori).collect()
    }
}
