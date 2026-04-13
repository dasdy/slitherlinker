use varisat::{CnfFormula, ExtendFormula, Solver};
use crate::data::solution::Solution;
use crate::parse::Cell;
use crate::solve_common::{handle_ok, solve_form_conditions};

pub fn solve(grid: Vec<Vec<Cell>>, pre_solve: bool, prefix: &str) -> Option<Vec<Solution>> {
    let mut formula = CnfFormula::new();


    let (puzzle, facts, base_edges) = solve_form_conditions(
        grid, pre_solve, &mut formula, prefix);

    let mut s = Solver::default();
    s.add_formula(&formula);

    let mut solutions = vec![];
    let mut counter = 0;
    let mut last_solution = None;
    println!("{prefix}facts found: {}", facts.len());
    while counter < 10000 {
        let has_solutions = s.solve().unwrap();
        if counter % 500 == 0 {
            println!("{prefix}attempt {counter}");
        }
        if has_solutions {
            let current_solution = s.model().unwrap();
            let (new_clause, approx_solution) = handle_ok(
                &puzzle,
                &facts,
                &base_edges,
                &mut solutions,
                current_solution.as_slice(),
                prefix,
            );
            if approx_solution.is_some() { last_solution = approx_solution }
            s.add_clause(&new_clause);
            if !solutions.is_empty() { break; }
        } else {
            println!("{prefix}No more solutions!");
            break;
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
    use crate::data::puzzle::Puzzle;
    use super::solve;
    use super::Edge;

    #[test]
    fn solves_simplest_2x2() {
        let s = solve(vec![vec![3, 2], vec![-1, -1]], false, "");
        assert!(s.is_some());
        let val = s.unwrap();
        assert_eq!(val.len(), 1);
    }

    #[test]
    fn handles_bad_puzzle() {
        let s = solve(
            vec![vec![0, 0], vec![0, 2]], false, "");

        assert!(s.is_some());
        assert_eq!(s.unwrap().len(), 0);
    }
}