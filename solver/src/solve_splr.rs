use std::collections::HashMap;


use splr::solver::*;
use splr::types::*;
use crate::adapter::SplrRules;


use crate::data::pattern::Edge;
use crate::data::puzzle::Puzzle;
use crate::data::solution::{Solution, _format_edges};
use crate::parse::Cell;
use crate::patterns::find_facts;
use crate::solve_common::{cell_clauses, edge_clauses, single_loop};



// TODO try extracting common code with verisat solve
pub fn solve_splr(grid: Vec<Vec<Cell>>, pre_solve: bool) -> Option<Vec<Solution>> {
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

    let mut formula: SplrRules = SplrRules::new();

    for (&k, &v) in facts.iter() {
        let edge = if v { Lit::from(k) } else { Lit::from(!k) };
        formula.push(vec![edge]);
    }

    cell_clauses(&p, &facts, &mut formula);
    edge_clauses(&p, &facts, &mut formula);

    let mut final_formula: Vec<Vec<i32>> = formula
        .iter()
        .map(|is| is.iter().map(|x| x.into()).collect())
        .collect();

    let mut solutions = vec![];
    let mut counter = 0;
    let mut last_solution = None;
    println!("facts found: {}", facts.len());
    while counter < 10000 {
        // let has_solutions = result.is_ok() && result.unwrap();
        if counter % 500 == 0 {
            println!("attempt {counter}");
        }
        let solve_result = Certificate::try_from(final_formula.clone());
        match solve_result {
            Ok(Certificate::SAT(sol)) => {
                let edges: Vec<Edge> = sol
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
                    solutions.push(solution);
                } else {
                    last_solution = Some(solution);
                }
                let new_clause: Vec<i32> = sol.iter().map(|&l| -l).collect();
                final_formula.push(new_clause);
            }
            Ok(Certificate::UNSAT) => {
                println!("No more solutions!");
                break;
            }
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        };
        counter += 1;
    }
    if solutions.is_empty() {
        println!("no proper solutions, well here's last thing:");
        match last_solution {
            Some(s) => solutions.push(s),
            None => println!("oh well"),
        };
    }
    Some(solutions)
}
