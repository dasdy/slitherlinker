use std::collections::HashMap;
use varisat::{CnfFormula, ExtendFormula, Lit, Solver};

use crate::data::pattern::Edge;
use crate::data::puzzle::Puzzle;
use crate::data::solution::{Solution, _format_edges};
use crate::parse::Cell;
use crate::patterns::find_facts;
use crate::solve_common::{
    clause_one, clause_three, clause_two, clause_zero, loop_four, loop_three, loop_two, single_loop,
};

fn cell_clauses(p: &Puzzle, facts: &HashMap<usize, bool>, formula: &mut CnfFormula) {
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
            let lits = (
                Lit::from_index(edges.0, true),
                Lit::from_index(edges.1, true),
                Lit::from_index(edges.2, true),
                Lit::from_index(edges.3, true),
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
                formula.add_clause(&c);
            }
        }
    }
}

fn edge_clauses(p: &Puzzle, facts: &HashMap<usize, bool>, formula: &mut CnfFormula) {
    for i in 0..=p.xsize {
        for j in 0..=p.ysize {
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
                formula.add_clause(
                    &c.iter()
                        .map(|&x| Lit::from_index(x, true))
                        .collect::<Vec<Lit>>(),
                );
            }
        }
    }
}

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
