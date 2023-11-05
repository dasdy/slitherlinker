

use std::collections::HashMap;
use std::fmt;

use crate::data::pattern::Edge;
use crate::data::puzzle::Puzzle;

pub struct Solution {
    pub puzzle: Puzzle,
    pub edges: Vec<Edge>,
    pub edges_pre_solve: Vec<Edge>,
    pub facts: HashMap<usize, bool>,
}

pub fn _format_edges(puzzle: &Puzzle, edges: &[Edge]) -> String {
    let mut res = String::new();
    for i in 0..puzzle.xsize {
        // top edges
        for j in 0..puzzle.ysize {
            let ix = puzzle.edge_ix(i, j, true);
            match edges.get(ix) {
                Some(Edge::Filled) => res.push_str(".-"),
                Some(Edge::Empty) => res.push_str(".x"),
                _ => res.push_str(". "),
            }
        }
        res.push_str(" \n");

        // vertical edges
        for j in 0..puzzle.ysize {
            let ix = puzzle.edge_ix(i, j, false);
            match edges.get(ix) {
                Some(Edge::Filled) => res.push('|'),
                Some(Edge::Empty) => res.push('x'),
                _ => res.push(' '),
            }

            if puzzle.cells[i][j] >= 0 {
                res.push_str(format!("{}", puzzle.cells[i][j]).as_str());
            } else {
                res.push(' ');
            }
        }

        match edges.get(puzzle.edge_ix(i, puzzle.ysize, false)) {
            Some(Edge::Filled) => res.push('|'),
            Some(Edge::Empty) => res.push('x'),
            _ => res.push(' '),
        }

        res.push('\n');
    }

    for j in 0..puzzle.ysize {
        match edges.get(puzzle.edge_ix(puzzle.xsize, j, true)) {
            Some(Edge::Filled) => res.push_str(" -"),
            Some(Edge::Empty) => res.push_str(" x"),
            _ => res.push_str("  "),
        }
    }
    res.push_str(" \n");

    res
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s0 = String::new();
        s0.push_str("Input puzzle:\n");
        s0.push_str(_format_edges(&self.puzzle, &self.edges_pre_solve).as_str());
        
        s0.push_str("Solved puzzle:\n");
        s0.push_str(_format_edges(&self.puzzle, &self.edges).as_str());
        write!(f, "{}", s0.as_str())
    }
}
