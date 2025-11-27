use crate::modules::tukang_ledeng::TukangLedeng;
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
}
