use std::collections::HashMap;

use splr::solver::*;
use splr::types::*;

use crate::data::pattern::Edge;
use crate::data::puzzle::Puzzle;
use crate::data::solution::{Solution, _format_edges};
use crate::parse::Cell;
use crate::patterns::find_facts;
use crate::solve_common::single_loop;
use crate::solve_common::{
    clause_one, clause_three, clause_two, clause_zero, loop_four, loop_three, loop_two,
};

pub type Rules = Vec<Vec<Lit>>;

// TODO rewrite this using sprl solver or something like that i dunno
fn cell_clauses(p: &Puzzle, facts: &HashMap<usize, bool>, formula: &mut Rules) {
    for i in 0..p.xsize {
        for j in 0..p.ysize {
            let condition = p.cells[i][j];
            if condition < 0 {
                continue;
            }
            let edges = p.edges_around_cell(i, j);
            if vec![edges.0, edges.1, edges.2, edges.3]
                .iter()
                .all(|i| facts.contains_key(i))
            {
                println!("Skipping cell clause: {condition} at [{i}][{j}]");
                continue;
            }
            // all set to true
            let lits = (
                Lit::from(edges.0),
                Lit::from(edges.1),
                Lit::from(edges.2),
                Lit::from(edges.3),
            );
            let v = match condition {
                0 => clause_zero(lits),
                1 => clause_one(lits),
                2 => clause_two(lits),
                3 => clause_three(lits),
                _ => vec![],
            };

            // println!("cell ({condition} [{i}][{j}]): {:?}", v);
            for c in v {
                // formula.add_clause(c);
                formula.push(c);
            }
        }
    }
}

// TODO rewrite this using sprl solver or something like that i dunno
fn edge_clauses(p: &Puzzle, facts: &HashMap<usize, bool>, formula: &mut Rules) {
    for i in 0..=p.xsize {
        for j in 0..=p.ysize {
            // TODO this should return correct lit from splr
            let es = p.edges_around_point(i, j);
            if es.iter().all(|&l| facts.contains_key(&l)) {
                println!("Skipping edge clauses for [{i}][{j}]");
                continue;
            }
            let clauses = match es.len() {
                2 => loop_two(es[0], es[1]),
                3 => loop_three(es[0], es[1], es[2]),
                4 => loop_four(es[0], es[1], es[2], es[3]),
                _ => panic!("???"),
            };

            // println!("loop: {} [{i}][{j}]: {:?}", es.len(), clauses);
            for c in clauses {
                // formula.add_clause(c);
                formula.push(c.iter().map(|&l| Lit::from(l as i32)).collect());
            }
        }
    }
}

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

    let mut formula: Rules = Rules::new();

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
