use std::collections::HashMap;
use std::fmt;

use crate::data::pattern::Edge;
use crate::data::puzzle::Puzzle;

pub const ANSI_RED: &str = "\x1b[31m";
/// Yellow background — visible on space characters (Unknown edges).
pub const ANSI_YELLOW_BG: &str = "\x1b[43m";
pub const ANSI_RESET: &str = "\x1b[0m";

/// Returns the visual (terminal) character count of a string, ignoring ANSI CSI escape sequences.
fn visual_length(s: &str) -> usize {
    let mut len = 0;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' && chars.peek() == Some(&'[') {
            chars.next(); // consume '['
            for c2 in chars.by_ref() {
                if c2.is_ascii_alphabetic() { break; }
            }
        } else {
            len += 1;
        }
    }
    len
}

fn render_edge_char(edges: &[Edge], ix: usize, filled: char, empty: char) -> char {
    match edges.get(ix) {
        Some(Edge::Filled) => filled,
        Some(Edge::Empty) => empty,
        _ => ' ',
    }
}

fn apply_color(c: char, color: Option<&'static str>) -> String {
    match color {
        Some(code) => format!("{}{}{}", code, c, ANSI_RESET),
        None => c.to_string(),
    }
}

/// Like `format_puzzle` but highlights specific edge indices.
/// `highlights` maps edge index → ANSI color code (e.g. `ANSI_RED`, `ANSI_YELLOW_BG`).
pub fn format_puzzle_diff(
    puzzle: &Puzzle,
    edges: &[Edge],
    highlights: &HashMap<usize, &'static str>,
) -> String {
    let mut res = String::new();
    for i in 0..puzzle.xsize {
        // top edges row
        for j in 0..puzzle.ysize {
            let ix = puzzle.edge_ix(i, j, true);
            res.push(cell_corner(puzzle, edges, i, j));
            let c = render_edge_char(edges, ix, '─', 'x');
            res.push_str(&apply_color(c, highlights.get(&ix).copied()));
        }
        res.push(cell_corner(puzzle, edges, i, puzzle.ysize));
        res.push('\n');

        // cell row with vertical edges
        for j in 0..puzzle.ysize {
            let ix = puzzle.edge_ix(i, j, false);
            let c = render_edge_char(edges, ix, '│', 'x');
            res.push_str(&apply_color(c, highlights.get(&ix).copied()));
            if puzzle.cells[i][j] >= 0 {
                res.push_str(&format!("{}", puzzle.cells[i][j]));
            } else {
                res.push(' ');
            }
        }
        let ix = puzzle.edge_ix(i, puzzle.ysize, false);
        let c = render_edge_char(edges, ix, '│', 'x');
        res.push_str(&apply_color(c, highlights.get(&ix).copied()));
        res.push('\n');
    }

    // bottom edge row
    for j in 0..puzzle.ysize {
        res.push(cell_corner(puzzle, edges, puzzle.xsize, j));
        let ix = puzzle.edge_ix(puzzle.xsize, j, true);
        let c = render_edge_char(edges, ix, '─', 'x');
        res.push_str(&apply_color(c, highlights.get(&ix).copied()));
    }
    res.push(cell_corner(puzzle, edges, puzzle.xsize, puzzle.ysize));
    res.push('\n');

    res
}

/// Renders two puzzle strings side-by-side with labels.
/// `col_visual_width` is the visual character width of each column (used for padding).
pub fn format_side_by_side(
    left: &str,
    right: &str,
    left_label: &str,
    right_label: &str,
    col_visual_width: usize,
) -> String {
    const SEP: &str = "  ";
    let mut result = String::new();

    let padding = col_visual_width.saturating_sub(visual_length(left_label));
    result.push_str(&format!("{}{}{}{}\n", left_label, " ".repeat(padding), SEP, right_label));

    let left_lines: Vec<&str> = left.lines().collect();
    let right_lines: Vec<&str> = right.lines().collect();
    let max_rows = left_lines.len().max(right_lines.len());

    for i in 0..max_rows {
        let left_line = left_lines.get(i).copied().unwrap_or("");
        let right_line = right_lines.get(i).copied().unwrap_or("");
        let vlen = visual_length(left_line);
        let pad = col_visual_width.saturating_sub(vlen);
        result.push_str(left_line);
        result.push_str(&" ".repeat(pad));
        result.push_str(SEP);
        result.push_str(right_line);
        result.push('\n');
    }

    result
}

pub struct Solution {
    pub puzzle: Puzzle,
    pub edges: Vec<Edge>,
    pub edges_pre_solve: Vec<Edge>,
    #[allow(dead_code)]
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
    format_puzzle_diff(puzzle, edges, &HashMap::new())
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
