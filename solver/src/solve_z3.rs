use std::collections::HashMap;

use z3::ast::Bool;
use z3::{SatResult, Solver};

use crate::data::pattern::Edge;
use crate::data::puzzle::Puzzle;
use crate::data::solution::Solution;
use crate::parse::Cell;
use crate::patterns::find_facts;
use crate::solve_common::{blocking_clause_edge_groups, find_loops_edges};

/// Z3 disjunct: literal that is true iff this edge differs from the given model value.
fn z3_edge_differs_lit(var: &Bool, model_filled: bool) -> Bool {
    if model_filled {
        !var
    } else {
        var.clone()
    }
}

fn assert_fact_units(solver: &Solver, vars: &[Bool], facts: &HashMap<usize, bool>) {
    for (&k, &v) in facts {
        if v {
            solver.assert(&vars[k]);
        } else {
            solver.assert(!&vars[k]);
        }
    }
}

fn assert_cell_constraints(solver: &Solver, p: &Puzzle, vars: &[Bool]) {
    for i in 0..p.xsize {
        for j in 0..p.ysize {
            let c = p.cells[i][j];
            if c < 0 {
                continue;
            }
            let (e0, e1, e2, e3) = p.edges_around_cell(i, j);
            let weighted = [
                (&vars[e0], 1),
                (&vars[e1], 1),
                (&vars[e2], 1),
                (&vars[e3], 1),
            ];
            solver.assert(Bool::pb_eq(&weighted, c as i32));
        }
    }
}

fn assert_vertex_constraints(solver: &Solver, p: &Puzzle, vars: &[Bool]) {
    for i in 0..=p.xsize {
        for j in 0..=p.ysize {
            let indices = p.edges_around_point(i, j);
            let weighted: Vec<(&Bool, i32)> = indices.iter().map(|&ix| (&vars[ix], 1)).collect();
            let exactly_zero = Bool::pb_eq(&weighted, 0);
            let exactly_two = Bool::pb_eq(&weighted, 2);
            solver.assert(Bool::or(&[exactly_zero, exactly_two]));
        }
    }
}

/// Same blocking policy as [`crate::solve_common::handle_ok_2`]: one OR per loop component.
fn assert_blocking_groups_z3(
    solver: &Solver,
    vars: &[Bool],
    edges: &[Edge],
    groups: &[Vec<usize>],
) {
    for g in groups {
        let clause: Vec<Bool> = g
            .iter()
            .map(|&i| z3_edge_differs_lit(&vars[i], edges[i] == Edge::Filled))
            .collect();
        solver.assert(Bool::or(&clause));
    }
}

/// Builds puzzle, facts, base edge paint, and one Z3 [`Bool`] per grid edge.
fn z3_slitherlink_instance(
    grid: Vec<Vec<Cell>>,
    pre_solve: bool,
    prefix: &str,
) -> (Puzzle, HashMap<usize, bool>, Vec<Edge>, Vec<Bool>, Solver) {
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

    let vars: Vec<Bool> = (0..num_edges)
        .map(|i| Bool::new_const(format!("e{i}")))
        .collect();

    let solver = Solver::new();
    assert_fact_units(&solver, &vars, &facts);
    assert_cell_constraints(&solver, &p, &vars);
    assert_vertex_constraints(&solver, &p, &vars);

    (p, facts, base_edges, vars, solver)
}

pub fn solve_z3(grid: Vec<Vec<Cell>>, pre_solve: bool, prefix: &str) -> Option<Vec<Solution>> {
    let (p, facts, base_edges, vars, solver) = z3_slitherlink_instance(grid, pre_solve, prefix);

    println!("{prefix}facts found: {}", facts.len());

    let num_edges = vars.len();
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
                    .map(
                        |var| match model.eval(var, true).and_then(|b| b.as_bool()) {
                            Some(true) => Edge::Filled,
                            _ => Edge::Empty,
                        },
                    )
                    .collect();

                let solution = Solution {
                    puzzle: p.clone(),
                    edges: edges.clone(),
                    edges_pre_solve: base_edges.clone(),
                    facts: facts.clone(),
                };

                if find_loops_edges(&p, &edges).len() == 1 {
                    println!("{prefix}WIN! found single-loop solution!");
                    solutions.push(solution);
                    break;
                } else {
                    last_solution = Some(solution);
                }

                let groups = blocking_clause_edge_groups(&p, num_edges, &edges);
                assert_blocking_groups_z3(&solver, &vars, &edges, &groups);
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
