use splr::types::Lit as SplrLit;
use varisat::{CnfFormula, ExtendFormula, Lit as VLit};
use crate::data::pattern::Edge;

pub trait SlitherlinkerLit {
    fn to_edge(&self) -> Edge;
    fn invert(&self) -> Self;
}

/// Adapter to enable writing generic solvers using different SAT engines
pub trait SlitherlinkerFormula<T: SlitherlinkerLit> {
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

impl SlitherlinkerLit for i32 {
    #[inline]
    fn to_edge(&self) -> Edge {
        if self.is_positive() {
            Edge::Filled
        } else {
            Edge::Empty
        }
    }

    #[inline]
    fn invert(&self) -> Self {
        -(*self)
    }
}

impl SlitherlinkerLit for VLit {
    #[inline]
    fn to_edge(&self) -> Edge {
        if self.is_positive() {
            Edge::Filled
        } else {
            Edge::Empty
        }
    }

    #[inline]
    fn invert(&self) -> Self {
        !(*self)
    }
}

impl SlitherlinkerLit for SplrLit {
    #[inline]
    fn to_edge(&self) -> Edge {
        let as_i32: i32 = self.into();
        as_i32.to_edge()
    }

    #[inline]
    fn invert(&self) -> Self {
        !(*self)
    }
}