use std::collections::HashMap;

use crate::data::baked_in_patterns::patterns;
use crate::data::pattern::Cell;
use crate::data::pattern::CellWindow;
use crate::data::pattern::Edge;
use crate::data::pattern::Horizontals;
use crate::data::pattern::PatternSolution;
use crate::data::pattern::Verticals;
use crate::data::puzzle::Puzzle;

use crate::data::solution::_format_edges;

pub fn find_facts(puzzle: &Puzzle) -> HashMap<usize, bool> {
    let mut res = HashMap::new();

    let patterns = patterns();

    let xsize = puzzle.xsize;
    let ysize = puzzle.ysize;
    let horizontals = (1 + xsize) * ysize;
    let verticals = xsize * (1 + ysize);

    let mut options = vec![Edge::Unknown; horizontals + verticals];

    let mut found_facts = true;
    let mut ctr = 0;
    while found_facts && ctr < 1000 {
        found_facts = false;
        ctr += 1;

        for i in -1..xsize as isize {
            for j in -1..ysize as isize {
                let window = mk_window(puzzle, i, j);
                let horizontals = mk_edge_hor_window(puzzle, &options, i, j);
                let verticals = mk_edge_vert_window(puzzle, &options, i, j);

                for (pname, p) in &patterns {
                    if p.try_match(&window, &horizontals, &verticals) {
                        let current_size = res.len();
                        update_things(&mut res, &mut options, p, puzzle, i, j);
                        if res.len() > current_size {
                            println!("found new {pname} at {i} {j}");
                            found_facts = true;

                            
                            let mut base_edges = vec![Edge::Unknown; (1 + xsize) * ysize + (1 + ysize) * xsize];
                            for (&k, &v) in res.iter() {
                                base_edges[k] = if v { Edge::Filled } else { Edge::Empty };
                            }

                            println!("after this step:\n{}", _format_edges(puzzle, &base_edges));
                        }
                    };
                }
            }
        }
    }

    res
}
fn update_things(
    res: &mut HashMap<usize, bool>,
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
                res.insert(edge_ix, hor_edge == Edge::Filled);
                opts[edge_ix] = hor_edge;
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
                res.insert(edge_ix, ver_edge == Edge::Filled);
                opts[edge_ix] = ver_edge;
            }
        }
    }
}

fn mk_window(p: &Puzzle, i: isize, j: isize) -> CellWindow {
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

fn edge_window_fetch(
    p: &Puzzle,
    edges: &[Edge],
    i: isize,
    j: isize,
    is_horizontal: bool,
) -> Edge {
    if i < 0 || j < 0 || i as usize >= p.xsize || j as usize >= p.ysize {
        return Edge::OutOfBounds;
    }
    edges[p.edge_ix(i as usize, j as usize, is_horizontal)]
}

fn mk_edge_hor_window(p: &Puzzle, edges: &[Edge], i: isize, j: isize) -> Horizontals {
    let mut res = [[Edge::Unknown; 3]; 2];
    res[0][0] = edge_window_fetch(p, edges, i, j - 1, true);
    res[0][1] = edge_window_fetch(p, edges, i, j, true);
    res[0][2] = edge_window_fetch(p, edges, i, j + 1, true);

    res[1][0] = edge_window_fetch(p, edges, i + 1, j - 1, true);
    res[1][1] = edge_window_fetch(p, edges, i + 1, j, true);
    res[1][2] = edge_window_fetch(p, edges, i + 1, j + 1, true);

    res
}

fn mk_edge_vert_window(p: &Puzzle, edges: &[Edge], i: isize, j: isize) -> Verticals {
    let mut res = [[Edge::Unknown; 2]; 3];

    res[0][0] = edge_window_fetch(p, edges, i - 1, j, false);
    res[0][1] = edge_window_fetch(p, edges, i - 1, j + 1, false);
    res[1][0] = edge_window_fetch(p, edges, i, j, false);
    res[1][1] = edge_window_fetch(p, edges, i, j + 1, false);
    res[2][0] = edge_window_fetch(p, edges, i + 1, j, false);
    res[2][1] = edge_window_fetch(p, edges, i + 1, j + 1, false);

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
        let p = Puzzle {
            cells: vec![],
            xsize: 10,
            ysize: 10,
        };
        update_things(&mut h, &mut edges, &threes_ortho, &p, 3, 4);
        assert_eq!(HashMap::from([(44, true), (34, true), (147, false)]), h);
        h.clear();
        update_things(&mut h, &mut edges, &threes_ortho, &p, 0, 0);
        assert_eq!(HashMap::from([(0, true), (10, true), (110, false)]), h);
    }
}
