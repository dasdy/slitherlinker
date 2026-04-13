use std::collections::HashMap;

use crate::data::baked_in_patterns::patterns;
use crate::data::baked_in_patterns::patterns_from_str;
use crate::data::pattern::Cell;
use crate::data::pattern::CellWindow;
use crate::data::pattern::Edge;
use crate::data::pattern::Horizontals;
use crate::data::pattern::PatternSolution;
use crate::data::pattern::Verticals;
use crate::data::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::data::solution::format_puzzle;

/// Facts are values for edges that we can deduce using patterns.
/// For example, if there are two orthogonally-connected '3' cells, they MUST contain edges
/// like this: |3|3|. This is a non-bruteforce part of the solution, where we are only limited
/// by how advanced the patterns are. Ideally, any puzzle that does not contain bifurcation,
/// should be solved only by deducing these facts.
pub fn find_facts(puzzle: &Puzzle, prefix: &str) -> HashMap<usize, bool> {
    #[allow(unused_variables)]
    let prefix = prefix;
    let mut facts_map = HashMap::new();

    // let patterns = patterns();
    let patterns = patterns_from_str();

    let horizontal_edge_count = (1 + puzzle.xsize) * puzzle.ysize;
    let vertical_edge_count = puzzle.xsize * (1 + puzzle.ysize);

    let mut options = vec![Edge::Unknown; horizontal_edge_count + vertical_edge_count];

    let mut found_facts = true;
    let mut ctr = 0;
    while found_facts && ctr < 1000 {
        found_facts = false;
        ctr += 1;

        for i in -1..puzzle.xsize as isize {
            for j in -1..puzzle.ysize as isize {
                let window = cell_window(puzzle, i, j);
                let hor_edges = horizontal_edge_window(puzzle, &options, i, j);
                let vert_edges = vertical_edge_window(puzzle, &options, i, j);

                for (pattern_name, pattern_solution) in &patterns {
                    let _ = pattern_name;
                    if pattern_solution.try_match(&window, &hor_edges, &vert_edges) {
                        let current_size = facts_map.len();
                        remember_facts(
                            &mut facts_map,
                            &mut options,
                            pattern_solution,
                            puzzle,
                            i,
                            j,
                        );
                        if facts_map.len() > current_size {
                            // println!("{prefix}found new {pattern_name} at {i} {j}");
                            found_facts = true;

                            let mut base_edges =
                                vec![Edge::Unknown; horizontal_edge_count + vertical_edge_count];
                            for (&k, &v) in facts_map.iter() {
                                base_edges[k] = if v { Edge::Filled } else { Edge::Empty };
                            }

                            // println!("{prefix}after this step:\n{}", format_puzzle(puzzle, &base_edges));
                        }
                    };
                }
            }
        }
    }

    facts_map
}

fn remember_facts(
    // map <edge_ix> -> <value>
    facts_map: &mut HashMap<usize, bool>,
    // list of all edges state
    opts: &mut [Edge],
    pattern: &PatternSolution,
    puzzle: &Puzzle,
    i: isize,
    j: isize,
) {
    for i_w in 0..2 {
        for j_w in 0..3 {
            let hor_j_ix = j + j_w - 1;
            let hor_edge = pattern.output.horizontals[i_w as usize][j_w as usize];
            if hor_edge != Edge::Empty && hor_edge != Edge::Filled {
                continue;
            }
            if hor_j_ix >= 0 && (i + i_w) >= 0 && (hor_j_ix as usize) < puzzle.ysize {
                let edge_ix = puzzle.edge_ix((i + i_w) as usize, hor_j_ix as usize, true);
                if let std::collections::hash_map::Entry::Vacant(e) = facts_map.entry(edge_ix) {
                    e.insert(hor_edge == Edge::Filled);
                    opts[edge_ix] = hor_edge;
                }
            }
        }
    }

    for i_w in 0..3 {
        for j_w in 0..2 {
            let ver_ix = i + i_w - 1;

            let ver_edge = pattern.output.verticals[i_w as usize][j_w as usize];
            if ver_edge != Edge::Empty && ver_edge != Edge::Filled {
                continue;
            }
            if ver_edge == Edge::OutOfBounds {
                continue;
            }

            if ver_ix >= 0 && j + j_w >= 0 && (ver_ix as usize) < puzzle.xsize {
                let edge_ix = puzzle.edge_ix(ver_ix as usize, (j + j_w) as usize, false);
                if let std::collections::hash_map::Entry::Vacant(e) = facts_map.entry(edge_ix) {
                    e.insert(ver_edge == Edge::Filled);
                    opts[edge_ix] = ver_edge;
                }
            }
        }
    }
}

fn cell_window(p: &Puzzle, i: isize, j: isize) -> CellWindow {
    let mut window = [[Cell::Any; 3]; 3];
    for i_w in 0..window.len() {
        for j_w in 0..window[0].len() {
            let i_ix = i + i_w as isize - 1;
            let j_ix = j + j_w as isize - 1;
            window[i_w][j_w] = if i_ix >= 0 && j_ix >= 0 {
                let (i, j) = (i_ix as usize, j_ix as usize);
                if i >= p.xsize || j >= p.ysize {
                    Cell::OutOfBounds
                } else {
                    match p.cells[i][j] {
                        0 => Cell::Zero,
                        1 => Cell::One,
                        2 => Cell::Two,
                        3 => Cell::Three,
                        -1 => Cell::Nothing,
                        _ => panic!("What is this value"),
                    }
                }
            } else {
                Cell::OutOfBounds
            }
        }
    }
    window
}

/// Similar to regular fetching edge by index from edges list, but in case of outbound
/// returns Edge::OutOfBounds value. Useful for creating windows over puzzle that are reaching
/// over the edge of puzzle.
fn window_safe_edge(p: &Puzzle, edges: &[Edge], i: isize, j: isize, is_horizontal: bool) -> Edge {
    if i < 0 || j < 0 {
        return Edge::OutOfBounds;
    }
    let (ui, uj) = (i as usize, j as usize);
    // Horizontal edges: i in [0, xsize], j in [0, ysize)
    // Vertical edges:   i in [0, xsize), j in [0, ysize]
    let out_of_bounds = if is_horizontal {
        ui > p.xsize || uj >= p.ysize
    } else {
        ui >= p.xsize || uj > p.ysize
    };
    if out_of_bounds {
        return Edge::OutOfBounds;
    }
    edges[p.edge_ix(ui, uj, is_horizontal)]
}

/// Make a window out of horizontal edges in the puzzle, with center at [i][j]
fn horizontal_edge_window(p: &Puzzle, edges: &[Edge], i: isize, j: isize) -> Horizontals {
    let mut res = [[Edge::Unknown; 3]; 2];
    res[0][0] = window_safe_edge(p, edges, i, j - 1, true);
    res[0][1] = window_safe_edge(p, edges, i, j, true);
    res[0][2] = window_safe_edge(p, edges, i, j + 1, true);

    res[1][0] = window_safe_edge(p, edges, i + 1, j - 1, true);
    res[1][1] = window_safe_edge(p, edges, i + 1, j, true);
    res[1][2] = window_safe_edge(p, edges, i + 1, j + 1, true);

    res
}

/// Make a window out of vertical edges in the puzzle, with center at [i][j]
fn vertical_edge_window(p: &Puzzle, edges: &[Edge], i: isize, j: isize) -> Verticals {
    let mut res = [[Edge::Unknown; 2]; 3];

    res[0][0] = window_safe_edge(p, edges, i - 1, j, false);
    res[0][1] = window_safe_edge(p, edges, i - 1, j + 1, false);
    res[1][0] = window_safe_edge(p, edges, i, j, false);
    res[1][1] = window_safe_edge(p, edges, i, j + 1, false);
    res[2][0] = window_safe_edge(p, edges, i + 1, j, false);
    res[2][1] = window_safe_edge(p, edges, i + 1, j + 1, false);

    res
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::pattern::Pattern;

    #[test]
    fn update_arbitrary_thing() {
        let threes_ortho = PatternSolution {
            output: Pattern {
                horizontals: [
                    [Edge::Any, Edge::Filled, Edge::Any],
                    [Edge::Any, Edge::Filled, Edge::Any],
                ],
                verticals: [
                    [Edge::Any, Edge::Any],
                    [Edge::Empty, Edge::Any],
                    [Edge::Any, Edge::Any],
                ],
            },
            input: Pattern {
                horizontals: [[Edge::Any; 3]; 2],
                verticals: [[Edge::Any; 2]; 3],
            },
            cells: [
                [Cell::Any, Cell::Three, Cell::Any],
                [Cell::Any, Cell::Three, Cell::Any],
                [Cell::Any, Cell::Any, Cell::Any],
            ],
        };

        let mut h: HashMap<usize, bool> = HashMap::new();
        let mut edges = vec![Edge::Empty; 220];
        let p = Puzzle::from(&[[-1; 10]; 10]);
        remember_facts(&mut h, &mut edges, &threes_ortho, &p, 3, 4);
        assert_eq!(HashMap::from([(44, true), (34, true), (147, false)]), h);
        h.clear();
        remember_facts(&mut h, &mut edges, &threes_ortho, &p, 0, 0);
        assert_eq!(HashMap::from([(0, true), (10, true), (110, false)]), h);
    }

    fn edges3x3() -> Vec<Edge> {
        Puzzle::edges(
            &[
                [Edge::Empty, Edge::Unknown, Edge::Empty],
                [Edge::Unknown, Edge::Empty, Edge::Empty],
                [Edge::Empty, Edge::Unknown, Edge::Empty],
                [Edge::Empty, Edge::Empty, Edge::Unknown],
            ],
            // Vertical edges
            &[
                [Edge::Filled, Edge::Any, Edge::Any, Edge::Any],
                [Edge::Any, Edge::Filled, Edge::Any, Edge::Any],
                [Edge::Any, Edge::Any, Edge::Filled, Edge::Filled],
            ],
        )
        .unwrap()
    }

    fn puzzle3x3() -> Puzzle {
        Puzzle::from(&[[1, 2, 3], [3, 1, 2], [-1, 3, 2]])
    }

    #[test]
    fn edge_vertical_window_simple() {
        assert_eq!(
            vertical_edge_window(&puzzle3x3(), edges3x3().as_slice(), 0, 0),
            Verticals::from([
                [Edge::OutOfBounds, Edge::OutOfBounds],
                [Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Filled]
            ])
        )
    }

    #[test]
    fn edge_horizontal_window_simple() {
        assert_eq!(
            horizontal_edge_window(&puzzle3x3(), edges3x3().as_slice(), 0, 0),
            Horizontals::from([
                [Edge::OutOfBounds, Edge::Empty, Edge::Unknown],
                [Edge::OutOfBounds, Edge::Unknown, Edge::Empty]
            ])
        )
    }

    #[test]
    fn test_cell_window() {
        assert_eq!(
            cell_window(&puzzle3x3(), 0, 0),
            CellWindow::from([
                [Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds],
                [Cell::OutOfBounds, Cell::One, Cell::Two],
                [Cell::OutOfBounds, Cell::Three, Cell::One],
            ])
        )
    }

    /// Regression test: boundary horizontal/vertical edges must not be misreported as
    /// OutOfBounds, which would cause patterns to fire incorrectly at the grid edge.
    ///
    /// For a 10×10 puzzle:
    ///   - Horizontal edges exist for i in [0, xsize=10], j in [0, ysize=10).
    ///     Before the fix, window_safe_edge returned OutOfBounds for i == xsize (the
    ///     bottom row of horizontal edges), which made patterns treating OutOfBounds as
    ///     Empty fire falsely and mark edge 109 (bottom-right horizontal) as Empty.
    ///   - Vertical edges exist for i in [0, xsize=10), j in [0, ysize=10].
    ///     Analogously, j == ysize was reported as OutOfBounds.
    #[test]
    fn test_boundary_edges_not_misreported_as_oob() {
        let p = Puzzle::from(&[[-1i8; 10]; 10]);
        let edges = vec![Edge::Unknown; (1 + p.xsize) * p.ysize + p.xsize * (1 + p.ysize)];

        // Bottom row of horizontal edges (i == xsize): must NOT be OutOfBounds.
        for j in 0..p.ysize as isize {
            let got = window_safe_edge(&p, &edges, p.xsize as isize, j, true);
            assert_eq!(
                got,
                Edge::Unknown,
                "horizontal edge at i=xsize={}, j={j} should be Unknown, not OutOfBounds",
                p.xsize
            );
        }
        // Row beyond the last is out of bounds.
        assert_eq!(
            window_safe_edge(&p, &edges, p.xsize as isize + 1, 0, true),
            Edge::OutOfBounds
        );

        // Rightmost column of vertical edges (j == ysize): must NOT be OutOfBounds.
        for i in 0..p.xsize as isize {
            let got = window_safe_edge(&p, &edges, i, p.ysize as isize, false);
            assert_eq!(
                got,
                Edge::Unknown,
                "vertical edge at i={i}, j=ysize={} should be Unknown, not OutOfBounds",
                p.ysize
            );
        }
        // Column beyond the last is out of bounds.
        assert_eq!(
            window_safe_edge(&p, &edges, 0, p.ysize as isize + 1, false),
            Edge::OutOfBounds
        );
    }

    /// Regression test: find_facts must not produce wrong deductions on the default puzzle.
    ///
    /// Before the fix, the bottom-right horizontal edge (index 109, i=10 j=9) and the
    /// rightmost vertical edge of the last row (index 219, i=9 j=10) were incorrectly
    /// marked as Empty because window_safe_edge mis-reported them as OutOfBounds.
    #[test]
    fn test_no_wrong_boundary_deductions_on_default_puzzle() {
        use crate::parse::from_string;

        let grid = from_string(
            "10x10d0:b1a2a22a32a1b22b2b2a23b212a22d222a31b2c12c2d331d013e1a2c2122a1b2a3b13a02a",
        )
        .unwrap();
        let xsize = grid.len();
        let ysize = grid[0].len();
        let p = Puzzle {
            cells: grid,
            xsize,
            ysize,
        };

        let facts = find_facts(&p, "");

        // Both edges are Filled in the true (SAT) solution.  The pre-solve must not
        // assert them as Empty (false) — that would make the SAT phase reach UNSAT.
        assert_ne!(
            facts.get(&109),
            Some(&false),
            "edge 109 (bottom-right horizontal, i=10 j=9) must not be deduced as Empty"
        );
        assert_ne!(
            facts.get(&219),
            Some(&false),
            "edge 219 (rightmost vertical of last row, i=9 j=10) must not be deduced as Empty"
        );
    }
}
