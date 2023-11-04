use std::collections::HashMap;
use varisat::{CnfFormula, ExtendFormula, Lit, Solver};

use crate::data::pattern::Edge;
use crate::data::puzzle::Puzzle;
use crate::data::solution::{Solution, _format_edges};
use crate::parse::Cell;
use crate::patterns::find_facts;
use crate::solve_common::{cell_clauses, edge_clauses, single_loop};

pub fn solve(grid: Vec<Vec<Cell>>, pre_solve: bool) -> Option<Vec<Solution>> {
    let xsize = grid.len();
    let ysize = grid[0].len();
    // let horizontals = (1 + xsize) * ysize;
    // let verticals = xsize * (1 + ysize);

    let p = Puzzle {
        cells: grid,
        xsize,
        ysize,
    };

    let facts = if pre_solve {
        find_facts(&p)
    } else {
        HashMap::new()
    };

    let mut base_edges = vec![Edge::Unknown; (1 + xsize) * ysize + (1 + ysize) * xsize];
    for (&k, &v) in facts.iter() {
        base_edges[k] = if v { Edge::Filled } else { Edge::Empty };
    }

    println!("After simplify:\n{}", _format_edges(&p, &base_edges));

    let mut formula = CnfFormula::new();

    for (&k, &v) in facts.iter() {
        formula.add_clause(&[Lit::from_index(k, v)]);
    }

    cell_clauses(&p, &facts, &mut formula);
    edge_clauses(&p, &facts, &mut formula);

    let mut s = Solver::default();
    s.add_formula(&formula);

    let mut sols = vec![];
    let mut counter = 0;
    let mut last_solution = None;
    println!("facts found: {}", facts.len());
    while counter < 10000 {
        let has_solutions = s.solve().unwrap();
        if counter % 500 == 0 {
            println!("attempt {counter}");
        }
        if has_solutions {
            let m = s.model().unwrap();
            let edges: Vec<Edge> = m
                .iter()
                .map(|&x| {
                    if x.is_positive() {
                        Edge::Filled
                    } else {
                        Edge::Empty
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
                println!("WIN! found single-loop solution! ");
                sols.push(solution);
            } else {
                last_solution = Some(solution);
            }
            let new_clause: Vec<Lit> = m.iter().map(|&l| !l).collect();
            s.add_clause(&new_clause);
        } else {
            println!("No more solutions!");
            break;
        }
        counter += 1;
    }
    if sols.is_empty() {
        println!("no proper solutions, well here's last thing:");
        match last_solution {
            Some(s) => sols.push(s),
            None => println!("oh well"),
        };
    }
    Some(sols)
}
