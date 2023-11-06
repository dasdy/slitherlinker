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

fn cell_corner(puzzle: &Puzzle, edges: &[Edge], i: usize, j: usize) -> char {
    let i_prev = i.checked_sub(1);
    let j_prev = j.checked_sub(1);

    let above = i_prev
        .and_then(|i| edges.get(puzzle.edge_ix(i, j, false)))
        .filter(|&&e| e == Edge::Filled);
    let below = edges.get(puzzle.edge_ix(i, j, false))
        .filter(|_| i <= puzzle.xsize && j <= puzzle.ysize)
        .filter(|&&e| e == Edge::Filled);
    let left = j_prev
        .and_then(|j| edges.get(puzzle.edge_ix(i, j, true)))
        .filter(|&&e| e == Edge::Filled);
    let right = edges.get(puzzle.edge_ix(i, j, true))
        .filter(|_| i <= puzzle.xsize && j < puzzle.ysize)
        .filter(|&&e| e == Edge::Filled);

    match (above.is_some(), below.is_some(), left.is_some(), right.is_some()) {
        (false, false, false, false) => '.',
        (false, false, true, true) => '─',
        (false, true, false, true) => '┌',
        (true, false, false, true) => '└',
        (false, true, true, false) => '┐',
        (true, true, false, false) => '│',
        (true, false, true, false) => '┘',

        (false, true, true, true) => '┬',
        (true, false, true, true) => '┴',
        (true, true, false, true) => '├',
        (true, true, true, false) => '┤',
        (true, true, true, true) => '┼',

        _ => '.'
        // panic!("{:?}, {:?}, {:?}, {:?}", above, below, left, right)
    }
}

pub fn format_puzzle(puzzle: &Puzzle, edges: &[Edge]) -> String {
    let mut res = String::new();
    for i in 0..puzzle.xsize {
        // top edges
        for j in 0..puzzle.ysize {
            let ix = puzzle.edge_ix(i, j, true);
            res.push(cell_corner(puzzle, edges, i, j));
            match edges.get(ix) {
                Some(Edge::Filled) => res.push('─'),
                Some(Edge::Empty) => res.push('x'),
                _ => res.push(' '),
            }
        }
        res.push(cell_corner(puzzle, edges, i, puzzle.ysize));
        res.push('\n');

        // vertical edges
        for j in 0..puzzle.ysize {
            let ix = puzzle.edge_ix(i, j, false);
            match edges.get(ix) {
                Some(Edge::Filled) => res.push('│'),
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
            Some(Edge::Filled) => res.push('│'),
            Some(Edge::Empty) => res.push('x'),
            _ => res.push(' '),
        }

        res.push('\n');
    }

    for j in 0..puzzle.ysize {
        res.push(cell_corner(puzzle, edges, puzzle.xsize, j));
        match edges.get(puzzle.edge_ix(puzzle.xsize, j, true)) {
            Some(Edge::Filled) => res.push('─'),
            Some(Edge::Empty) => res.push('x'),
            _ => res.push(' '),
        }
    }
    res.push(cell_corner(puzzle, edges, puzzle.xsize, puzzle.ysize));
    res.push('\n');

    res
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s0 = String::new();
        s0.push_str("Input puzzle:\n");
        s0.push_str(format_puzzle(&self.puzzle, &self.edges_pre_solve).as_str());

        s0.push_str("Solved puzzle:\n");
        s0.push_str(format_puzzle(&self.puzzle, &self.edges).as_str());
        write!(f, "{}", s0.as_str())
    }
}
