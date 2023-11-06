use varisat::{CnfFormula, ExtendFormula, Lit, Solver};
use crate::data::pattern::Edge;
use crate::data::solution::Solution;
use crate::parse::Cell;
use crate::solve_common::{single_loop, solve_form_conditions};

pub fn solve(grid: Vec<Vec<Cell>>, pre_solve: bool) -> Option<Vec<Solution>> {
    let mut formula = CnfFormula::new();


    let (p, facts, base_edges) = solve_form_conditions(
        grid, pre_solve, &mut formula);

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
}