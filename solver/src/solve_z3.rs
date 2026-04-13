use std::collections::HashMap;

use z3::ast::Bool;
use z3::{SatResult, Solver};

use crate::data::pattern::Edge;
use crate::data::puzzle::Puzzle;
use crate::data::solution::Solution;
use crate::parse::Cell;
use crate::patterns::find_facts;
use crate::solve_common::single_loop;

pub fn solve_z3(grid: Vec<Vec<Cell>>, pre_solve: bool, prefix: &str) -> Option<Vec<Solution>> {
    let xsize = grid.len();
    let ysize = grid[0].len();
    let p = Puzzle {
        cells: grid,
        xsize,
        ysize,
    };

    let facts = if pre_solve {
        find_facts(&p, prefix)
    } else {
        HashMap::new()
    };

    let num_edges = (1 + xsize) * ysize + (1 + ysize) * xsize;
    let mut base_edges = vec![Edge::Unknown; num_edges];
    for (&k, &v) in &facts {
        base_edges[k] = if v { Edge::Filled } else { Edge::Empty };
    }

    // One boolean variable per edge.
    let vars: Vec<Bool> = (0..num_edges)
        .map(|i| Bool::new_const(format!("e{i}")))
        .collect();

    let solver = Solver::new();

    // Assert pre-solve facts as unit clauses.
    for (&k, &v) in &facts {
        if v {
            solver.assert(&vars[k]);
        } else {
            solver.assert(!&vars[k]);
        }
    }

    // Cell constraints: exactly N edges filled around each numbered cell.
    for i in 0..xsize {
        for j in 0..ysize {
            let c = p.cells[i][j];
            if c < 0 {
                continue;
            }
            let (e0, e1, e2, e3) = p.edges_around_cell(i, j);
            let weighted = [(&vars[e0], 1), (&vars[e1], 1), (&vars[e2], 1), (&vars[e3], 1)];
            solver.assert(&Bool::pb_eq(&weighted, c as i32));
        }
    }

    // Vertex constraints: exactly 0 or 2 filled edges at each grid point.
    for i in 0..=xsize {
        for j in 0..=ysize {
            let indices = p.edges_around_point(i, j);
            let weighted: Vec<(&Bool, i32)> = indices.iter().map(|&ix| (&vars[ix], 1)).collect();
            let exactly_zero = Bool::pb_eq(&weighted, 0);
            let exactly_two = Bool::pb_eq(&weighted, 2);
            solver.assert(&Bool::or(&[exactly_zero, exactly_two]));
        }
    }

    println!("{prefix}facts found: {}", facts.len());

    let mut solutions = vec![];
    let mut counter = 0;
    let mut last_solution = None;

    while counter < 10000 {
        if counter % 500 == 0 {
            println!("{prefix}attempt {counter}");
        }

        match solver.check() {
            SatResult::Sat => {
                let model = solver.get_model().unwrap();

                let edges: Vec<Edge> = vars
                    .iter()
                    .map(|var| {
                        match model.eval(var, true).and_then(|b| b.as_bool()) {
                            Some(true) => Edge::Filled,
                            _ => Edge::Empty,
                        }
                    })
                    .collect();

                let solution = Solution {
                    puzzle: p.clone(),
                    edges: edges.clone(),
                    edges_pre_solve: base_edges.clone(),
                    facts: facts.clone(),
                };

                if single_loop(&p, &edges) {
                    println!("{prefix}WIN! found single-loop solution!");
                    solutions.push(solution);
                    break;
                } else {
                    last_solution = Some(solution);
                }

                // Blocking clause: at least one edge must differ from this assignment.
                let blocking: Vec<Bool> = vars
                    .iter()
                    .enumerate()
                    .map(|(i, var)| {
                        if edges[i] == Edge::Filled {
                            !var
                        } else {
                            var.clone()
                        }
                    })
                    .collect();
                solver.assert(&Bool::or(&blocking));
            }
            SatResult::Unsat => {
                println!("{prefix}No more solutions!");
                break;
            }
            SatResult::Unknown => {
                println!("{prefix}Z3 returned unknown!");
                break;
            }
        }

        counter += 1;
    }

    if solutions.is_empty() {
        println!("{prefix}no proper solutions, well here's last thing:");
        match last_solution {
            Some(s) => solutions.push(s),
            None => println!("{prefix}oh well"),
        };
    }

    Some(solutions)
}

#[cfg(test)]
mod test {
    use super::solve_z3;

    #[test]
    fn solves_simplest_2x2() {
        let s = solve_z3(vec![vec![3, 2], vec![-1, -1]], false, "");
        assert!(s.is_some());
        let val = s.unwrap();
        assert_eq!(val.len(), 1);
    }

    #[test]
    fn handles_bad_puzzle() {
        let s = solve_z3(vec![vec![0, 0], vec![0, 2]], false, "");
        assert!(s.is_some());
        assert_eq!(s.unwrap().len(), 0);
    }
}
