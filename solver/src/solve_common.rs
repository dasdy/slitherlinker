use crate::adapter::{SlitherlinkerFormula, SlitherlinkerLit};
use crate::data::pattern::Edge;
use crate::data::puzzle::Puzzle;
use crate::data::solution::{format_puzzle, Solution};
use crate::parse::Cell;
use crate::patterns::find_facts;
use std::collections::HashMap;
use std::{
    collections::{HashSet, LinkedList},
    ops::Not,
};

pub fn loop_two<T>(a: T, b: T) -> Vec<Vec<T>>
where
    T: Not<Output = T> + Copy,
{
    vec![vec![a, !b], vec![!a, b]]
}

pub fn loop_three<T>(a: T, b: T, c: T) -> Vec<Vec<T>>
where
    T: Not<Output = T> + Copy,
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
    T: Not<Output = T> + Copy,
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
    T: Not<Output = T>,
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
    T: Not<Output = T> + Copy,
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
    T: Not<Output = T> + Copy,
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
    T: Not<Output = T> + Copy,
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

/// Connected components of filled edges (each component is one closed polyline loop).
pub fn find_loops_edges(puzzle: &Puzzle, edges: &[Edge]) -> LinkedList<Vec<usize>> {
    let all_filled: HashSet<usize> = edges
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == Edge::Filled)
        .map(|(i, _)| i)
        .collect();

    if all_filled.is_empty() {
        return LinkedList::new();
    }

    let mut visited: HashSet<usize> = HashSet::new();
    let mut loops: LinkedList<Vec<usize>> = LinkedList::new();

    while let Some(&start) = all_filled.difference(&visited).next() {
        let mut current_loop: Vec<usize> = Vec::new();
        let mut queue: LinkedList<usize> = LinkedList::new();
        queue.push_back(start);

        while let Some(item) = queue.pop_front() {
            if visited.contains(&item) {
                continue;
            }
            visited.insert(item);
            current_loop.push(item);

            for n in puzzle.edges_around_edge(item) {
                if n < edges.len() && edges[n] == Edge::Filled && !visited.contains(&n) {
                    queue.push_back(n);
                }
            }
        }

        loops.push_back(current_loop);
    }

    loops
}

/// Edge-index groups for blocking clauses: one group per filled loop, or one global group
/// when there are no filled loops (same policy as [`handle_ok_2`]).
pub fn blocking_clause_edge_groups(
    puzzle: &Puzzle,
    num_edges: usize,
    edges: &[Edge],
) -> Vec<Vec<usize>> {
    let loops = find_loops_edges(puzzle, edges);
    let mut groups: Vec<Vec<usize>> = if loops.is_empty() {
        vec![(0..num_edges).collect()]
    } else {
        loops.iter().filter(|lp| !lp.is_empty()).cloned().collect()
    };
    if groups.is_empty() {
        groups = vec![(0..num_edges).collect()];
    }
    groups
}

pub fn single_loop_edge(puzzle: &Puzzle, edges: &[Edge]) -> bool {
    find_loops_edges(puzzle, edges).len() == 1
}

pub fn cell_clauses<T: SlitherlinkerLit + Not<Output = T> + Copy>(
    p: &Puzzle,
    facts: &HashMap<usize, bool>,
    formula: &mut impl SlitherlinkerFormula<T>,
    prefix: &str,
) {
    let _ = prefix;
    for i in 0..p.xsize {
        for j in 0..p.ysize {
            let condition = p.cells[i][j];
            if condition < 0 {
                continue;
            }
            let edges = p.edges_around_cell(i, j);
            if [edges.0, edges.1, edges.2, edges.3]
                .iter()
                .all(|i| facts.contains_key(i))
            {
                // println!("{prefix}Skipping cell clause: {condition} at [{i}][{j}]");
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

pub fn edge_clauses<T: SlitherlinkerLit + Not<Output = T> + Copy>(
    p: &Puzzle,
    facts: &HashMap<usize, bool>,
    formula: &mut impl SlitherlinkerFormula<T>,
    prefix: &str,
) {
    let _ = prefix;
    for i in 0..=p.xsize {
        for j in 0..=p.ysize {
            // TODO this should return correct lit from splr
            let edges = p.edges_around_point(i, j);
            let es = edges
                .iter()
                .map(|&x| formula.pure_lit(x))
                .collect::<Vec<T>>();

            if edges.iter().all(|&l| facts.contains_key(&(l))) {
                // println!("{prefix}Skipping edge clauses for [{i}][{j}]");
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
pub fn solve_form_conditions<T: SlitherlinkerLit + Not<Output = T> + Copy>(
    grid: Vec<Vec<Cell>>,
    pre_solve: bool,
    formula: &mut impl SlitherlinkerFormula<T>,
    prefix: &str,
) -> (Puzzle, HashMap<usize, bool>, Vec<Edge>) {
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

    if prefix == "varisat" {
        println!(
            "{prefix}After simplify:\n{}",
            format_puzzle(&p, &base_edges)
        );
    }

    for (&k, &v) in facts.iter() {
        let l = formula.pure_lit(k);
        formula.append_clause(vec![if v { l } else { l.invert() }]);
    }

    cell_clauses(&p, &facts, formula, prefix);
    edge_clauses(&p, &facts, formula, prefix);
    (p, facts, base_edges)
}

pub fn handle_ok_2<T: SlitherlinkerLit>(
    puzzle: &Puzzle,
    facts: &HashMap<usize, bool>,
    base_edges: &[Edge],
    solutions: &mut Vec<Solution>,
    solution_vector: &[T],
    prefix: &str,
) -> (Vec<Vec<T>>, Option<Solution>) {
    let edges: Vec<Edge> = solution_vector.iter().map(|x| x.to_edge()).collect();
    let solution = Solution {
        puzzle: puzzle.clone(),
        edges: edges.clone(),
        edges_pre_solve: base_edges.to_vec(),
        facts: facts.clone(),
    };
    let mut last_solution = None;
    let loops = find_loops_edges(puzzle, &edges);
    if loops.len() == 1 {
        println!("{prefix}WIN! found single-loop solution!");
        solutions.push(solution);
    } else {
        last_solution = Some(solution);
    }

    let groups = blocking_clause_edge_groups(puzzle, solution_vector.len(), &edges);
    let new_clauses: Vec<Vec<T>> = groups
        .iter()
        .map(|ixs| ixs.iter().map(|&e| solution_vector[e].invert()).collect())
        .collect();
    (new_clauses, last_solution)
}
