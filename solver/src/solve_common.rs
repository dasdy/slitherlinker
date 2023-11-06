use std::{
    collections::{HashSet, LinkedList},
    ops::{Not, Sub},
};
use std::collections::HashMap;
use crate::adapter::{SlitherlinkerFormula, SlitherlinkerLit};
use crate::parse::Cell;
use crate::data::pattern::Edge;
use crate::data::puzzle::Puzzle;
use crate::data::solution::{format_puzzle, Solution};
use crate::patterns::find_facts;

pub fn loop_two<T>(a: T, b: T) -> Vec<Vec<T>>
    where
        T: Not<Output=T> + Copy,
{
    vec![vec![a, !b], vec![!a, b]]
}

pub fn loop_three<T>(a: T, b: T, c: T) -> Vec<Vec<T>>
    where
        T: Not<Output=T> + Copy,
{
    vec![
        vec![!a, !b, !c],
        vec![!a, b, c],
        vec![a, !b, c],
        vec![a, b, !c],
    ]
}

pub fn loop_four<T>(a: T, b: T, c: T, d: T) -> Vec<Vec<T>>
    where
        T: Not<Output=T> + Copy,
{
    vec![
        vec![a, b, c, !d],
        vec![a, b, !c, d],
        vec![a, !b, c, d],
        vec![!a, b, c, d],
        vec![!a, !b, !d],
        vec![!b, !c, !d],
        vec![!a, !c, !d],
        vec![!a, !b, !c],
    ]
}

pub fn clause_zero<T>(edges: (T, T, T, T)) -> Vec<Vec<T>>
    where
        T: Not<Output=T>,
{
    vec![
        vec![!edges.0],
        vec![!edges.1],
        vec![!edges.2],
        vec![!edges.3],
    ]
}

pub fn clause_one<T>(edges: (T, T, T, T)) -> Vec<Vec<T>>
    where
        T: Not<Output=T> + Copy,
{
    let (a, b, c, d) = edges;

    // Did a Karno map for those things
    vec![
        vec![a, b, c, d],
        vec![!a, !b],
        vec![!c, !d],
        vec![!a, !d],
        vec![!c, !a],
        vec![!b, !c],
        vec![!d, !b],
    ]
}

pub fn clause_two<T>(edges: (T, T, T, T)) -> Vec<Vec<T>>
    where
        T: Not<Output=T> + Copy,
{
    let (a, b, c, d) = edges;

    // Did a Karno map for those things
    vec![
        vec![a, b, c],
        vec![a, c, d],
        vec![a, b, !c, d],
        vec![!a, b, c, d],
        vec![!a, !b, !d],
        vec![!c, !b, !d],
        vec![!a, !b, !c],
        vec![!c, !a, !d],
    ]
}

pub fn clause_three<T>(edges: (T, T, T, T)) -> Vec<Vec<T>>
    where
        T: Not<Output=T> + Copy,
{
    let (a, b, c, d) = edges;

    // Did a Karno map for those things
    vec![
        vec![a, b],
        vec![c, d],
        vec![a, c],
        vec![d, b],
        vec![a, d],
        vec![!a, b, c],
        vec![!a, !b, !c, !d],
    ]
}

pub fn single_loop(puzzle: &Puzzle, edges: &[Edge]) -> bool {
    let all_edge_indices: HashSet<usize> = HashSet::from_iter(
        edges
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == Edge::Filled)
            .map(|(i, &_)| i),
    );
    if all_edge_indices.is_empty() {
        return false;
    }
    let mut visited_edges: HashSet<usize> = HashSet::new();
    let mut queue: LinkedList<usize> = LinkedList::new();
    queue.push_back(*all_edge_indices.iter().min().unwrap());

    while !queue.is_empty() {
        let item = queue.pop_front().unwrap();
        visited_edges.insert(item);

        let neighbors: Vec<usize> = puzzle.edges_around_edge(item);

        for n in neighbors {
            if n >= edges.len() {
                continue;
            }
            if edges[n] == Edge::Filled && !visited_edges.contains(&n) {
                queue.push_back(n);
            }
        }
    }

    all_edge_indices.sub(&visited_edges).is_empty()
}

pub fn cell_clauses<T: SlitherlinkerLit>(
    p: &Puzzle, facts: &HashMap<usize, bool>, formula: &mut impl SlitherlinkerFormula<T>,
)
    where
        T: Not<Output=T> + Copy, {
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
                formula.pure_lit(edges.0),
                formula.pure_lit(edges.1),
                formula.pure_lit(edges.2),
                formula.pure_lit(edges.3),
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
                formula.append_clause(c);
            }
        }
    }
}

pub fn edge_clauses<T: SlitherlinkerLit>(
    p: &Puzzle, facts: &HashMap<usize, bool>, formula: &mut impl SlitherlinkerFormula<T>)
    where
        T: Not<Output=T> + Copy, {
    for i in 0..=p.xsize {
        for j in 0..=p.ysize {
            // TODO this should return correct lit from splr
            let edges = p.edges_around_point(i, j);
            let es = edges
                .iter()
                .map(|&x| formula.pure_lit(x))
                .collect::<Vec<T>>();

            if edges.iter().all(|&l| facts.contains_key(&(l)))
            {
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
                formula.append_clause(c);
            }
        }
    }
}

/// Using grid of cells,
/// 1. create an instance of Puzzle
/// 2. Find "facts" using patterns (only if pre_solve is true) as hashmap <edge-index: value>
/// 3. Use facts and cell-edge input to mutate input boolean formula
/// 4. Return the "base edges" vector - basically a materialized facts hashmap
pub fn solve_form_conditions<T: SlitherlinkerLit + Copy>(
    grid: Vec<Vec<Cell>>, pre_solve: bool, formula: &mut impl SlitherlinkerFormula<T>,
) -> (Puzzle, HashMap<usize, bool>, Vec<Edge>) where
    T: Not<Output=T> + Copy, {
    let xsize = grid.len();
    let ysize = grid[0].len();

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

    println!("After simplify:\n{}", format_puzzle(&p, &base_edges));

    for (&k, &v) in facts.iter() {
        let l = formula.pure_lit(k);
        formula.append_clause(vec![if v { l } else { l.invert() }]);
    }

    cell_clauses(&p, &facts, formula);
    edge_clauses(&p, &facts, formula);
    (p, facts, base_edges)
}

/// Handle OK result from solver
/// Two cases are possible: if the solution is a single loop, add it to solutions list
/// If not, save it as "last solution" and return as non-empty option
/// In any case, make the resulting solution a new formula for iterative solve
/// so that we can look for next better solutions (negate everything in this one to get new ones)
pub fn handle_ok<T: SlitherlinkerLit>(puzzle: &Puzzle,
                                      facts: &HashMap<usize, bool>,
                                      base_edges: &[Edge],
                                      solutions: &mut Vec<Solution>,
                                      solution_vector: &[T]) -> (Vec<T>, Option<Solution>) {
    let edges: Vec<Edge> = solution_vector
        .iter()
        .map(|x| x.to_edge())
        .collect();
    let solution = Solution {
        puzzle: puzzle.clone(),
        edges: edges.clone(),
        edges_pre_solve: base_edges.to_vec(),
        facts: facts.clone(),
    };
    let mut last_solution = None;
    if single_loop(puzzle, &edges) {
        println!("WIN! found single-loop solution!");
        solutions.push(solution);
    } else {
        last_solution = Some(solution);
    }

    let new_clause: Vec<T> = solution_vector.iter().map(|l| l.invert()).collect();
    (new_clause, last_solution)
}
