use varisat::{CnfFormula, ExtendFormula, Solver};
use crate::data::pattern::Edge;
use crate::data::solution::Solution;
use crate::parse::Cell;
use crate::solve_common::{handle_ok, solve_form_conditions};

pub fn solve(grid: Vec<Vec<Cell>>, pre_solve: bool) -> Option<Vec<Solution>> {
    let mut formula = CnfFormula::new();


    let (puzzle, facts, base_edges) = solve_form_conditions(
        grid, pre_solve, &mut formula);

    let mut s = Solver::default();
    s.add_formula(&formula);

    let mut solutions = vec![];
    let mut counter = 0;
    let mut last_solution = None;
    println!("facts found: {}", facts.len());
    while counter < 10000 {
        let has_solutions = s.solve().unwrap();
        if counter % 500 == 0 {
            println!("attempt {counter}");
        }
        if has_solutions {
            let current_solution = s.model().unwrap();
            let (new_clause, approx_solution) = handle_ok(
                &puzzle,
                &facts,
                &base_edges,
                &mut solutions,
                current_solution.as_slice(),
            );
            if approx_solution.is_some() { last_solution = approx_solution }
            s.add_clause(&new_clause);
        } else {
            println!("No more solutions!");
            break;
        }
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

#[cfg(test)]
mod test {
    use crate::data::puzzle::Puzzle;
    use super::solve;
    use super::Edge;

    #[test]
    fn solves_simplest_2x2() {
        let s = solve(vec![vec![3, 2], vec![-1, -1]], false);
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
        assert_eq!(val[1].edges,
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
    }

    #[test]
    fn handles_bad_puzzle() {
        let s = solve(
            vec![vec![0, 0], vec![0, 2]], false);

        assert!(s.is_some());
        assert_eq!(s.unwrap().len(), 0);
    }
}