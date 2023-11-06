use std::collections::HashMap;
use splr::solver::*;

use crate::adapter::{SplrRules};
use crate::data::pattern::Edge;
use crate::data::puzzle::Puzzle;
use crate::data::solution::{Solution};
use crate::parse::Cell;
use crate::solve_common::{single_loop, solve_form_conditions};

pub fn solve_splr(grid: Vec<Vec<Cell>>, pre_solve: bool) -> Option<Vec<Solution>> {
    let mut formula: SplrRules = SplrRules::new();

    let (p, facts, base_edges) = solve_form_conditions(
        grid, pre_solve, &mut formula);

    let mut final_formula: Vec<Vec<i32>> = formula
        .iter()
        .map(|is| is.iter().map(|x| x.into()).collect())
        .collect();

    let mut solutions = vec![];
    let mut counter = 0;
    let mut last_solution = None;
    println!("facts found: {}", facts.len());
    while counter < 10000 {
        if counter % 500 == 0 {
            println!("attempt {counter}");
        }
        let solve_result = Certificate::try_from(final_formula.clone());
        match solve_result {
            Ok(Certificate::SAT(sol)) => {
                let (new_clause, approx_solution) = handle_ok(
                    &p,
                    &facts,
                    &base_edges,
                    &mut solutions,
                    sol.as_slice(),
                );
                if approx_solution.is_some() { last_solution = approx_solution }
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

/// Handle OK result from solver
/// Two cases are possible: if the solution is a single loop, add it to solutions list
/// If not, save it as "last solution" and return as non-empty option
/// In any case, make the resulting solution a new formula for iterative solve
/// so that we can look for next better solutions (negate everything in this one to get new ones)
fn handle_ok(puzzle: &Puzzle,
             facts: &HashMap<usize, bool>,
             base_edges: &[Edge],
             solutions: &mut Vec<Solution>,
             solution_vector: &[i32]) -> (Vec<i32>, Option<Solution>) {
    let edges: Vec<Edge> = solution_vector
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
        puzzle: puzzle.clone(),
        edges: edges.clone(),
        edges_pre_solve: base_edges.to_vec(),
        facts: facts.clone(),
    };
    let mut last_solution = None;
    if single_loop(&puzzle, &edges) {
        println!("WIN! found single-loop solution! ");
        solutions.push(solution);
    } else {
        last_solution = Some(solution);
    }

    let new_clause: Vec<i32> = solution_vector.iter().map(|&l| -l).collect();
    (new_clause, last_solution)
}

#[cfg(test)]
mod test {
    use crate::data::puzzle::Puzzle;
    use super::solve_splr;
    use super::Edge;

    #[test]
    fn solves_simplest_2x2() {
        let s = solve_splr(vec![vec![3, 2], vec![-1, -1]], false);
        assert!(s.is_some());
        let val = s.unwrap();
        assert_eq!(val.len(), 2);
        /*
        .-.x
        |3|2x
        .x.-
        | x |
         - -
         */
        assert_eq!(val[0].edges,
                   Puzzle::edges(
                       &[
                           [Edge::Filled, Edge::Empty],
                           [Edge::Empty, Edge::Filled],
                           [Edge::Filled, Edge::Filled]],
                       &[
                           [Edge::Filled, Edge::Filled, Edge::Empty],
                           [Edge::Filled, Edge::Empty, Edge::Filled]
                       ],
                   ).unwrap()
        );
        /*
        .-.-
        |3x2|
        .-.x
        x | |
         x -
         */
        assert_eq!(val[1].edges,
                   Puzzle::edges(
                       &[[Edge::Filled, Edge::Filled],
                           [Edge::Filled, Edge::Empty],
                           [Edge::Empty, Edge::Filled]],
                       // vertical edges
                       &[[Edge::Filled, Edge::Empty, Edge::Filled],
                           [Edge::Empty, Edge::Filled, Edge::Filled]],
                   ).unwrap());
    }
}