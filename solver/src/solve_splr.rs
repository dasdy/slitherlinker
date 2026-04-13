use splr::solver::*;
use crate::adapter::SplrRules;
use crate::data::solution::Solution;
use crate::parse::Cell;
use crate::solve_common::{handle_ok, solve_form_conditions};

pub fn solve_splr(grid: Vec<Vec<Cell>>, pre_solve: bool, prefix: &str) -> Option<Vec<Solution>> {
    let mut formula: SplrRules = SplrRules::new();

    let (p, facts, base_edges) = solve_form_conditions(
        grid, pre_solve, &mut formula, prefix);

    let mut final_formula: Vec<Vec<i32>> = formula
        .iter()
        .map(|is| is.iter().map(|x| x.into()).collect())
        .collect();

    let mut solutions = vec![];
    let mut counter = 0;
    let mut last_solution = None;
    println!("{prefix}facts found: {}", facts.len());
    while counter < 10000 {
        if counter % 500 == 0 {
            println!("{prefix}attempt {counter}");
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
                    prefix,
                );
                if approx_solution.is_some() { last_solution = approx_solution }
                final_formula.push(new_clause);
                if !solutions.is_empty() { break; }
            }
            Ok(Certificate::UNSAT) => {
                println!("{prefix}No more solutions!");
                break;
            }
            Err(e) => {
                println!("{prefix}error: {}", e);
                break;
            }
        };
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
    use crate::data::pattern::Edge;
    use crate::data::puzzle::Puzzle;
    use super::solve_splr;

    #[test]
    fn solves_simplest_2x2() {
        let s = solve_splr(vec![vec![3, 2], vec![-1, -1]], false, "");
        assert!(s.is_some());
        let val = s.unwrap();
        assert_eq!(val.len(), 1);
    }

    #[test]
    fn handles_bad_puzzle() {
        let s = solve_splr(
            vec![vec![0, 0], vec![0, 2]], false, "");

        assert!(s.is_some());
        assert_eq!(s.unwrap().len(), 0);
    }
}