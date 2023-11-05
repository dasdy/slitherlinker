use std::ops::Not;
use splr::types::Lit as SplrLit;
use varisat::{CnfFormula, ExtendFormula, Lit as VLit};

/// Adapter to enable writing generic solvers using different SAT engines
pub trait SlitherlinkerFormula<T>
    where
        T: Not<Output=T> + Copy, {
    fn append_clause(&mut self, clause: Vec<T>);
    fn pure_lit(&self, ix: usize) -> T;
}

impl SlitherlinkerFormula<VLit> for CnfFormula {
    fn append_clause(&mut self, clause: Vec<VLit>) {
        self.add_clause(clause.as_slice())
    }
    fn pure_lit(&self, ix: usize) -> VLit {
        VLit::from_index(ix, true)
    }
}


pub type SplrRules = Vec<Vec<SplrLit>>;

impl SlitherlinkerFormula<SplrLit> for SplrRules {
    fn append_clause(&mut self, clause: Vec<SplrLit>) {
        self.append(&mut vec![clause])
    }

    fn pure_lit(&self, ix: usize) -> SplrLit {
        SplrLit::from(1 + ix as i32)
    }
}