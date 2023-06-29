use std::{
    collections::{HashMap, HashSet, LinkedList},
    fmt,
    ops::Sub,
};
use varisat::{CnfFormula, ExtendFormula, Lit, Solver};

use crate::parse::Cell;
use crate::patterns::find_facts;
use crate::data::pattern::Edge;
use crate::data::puzzle::Puzzle;

pub struct Solution {
    pub puzzle: Puzzle,
    pub edges: Vec<Edge>,
    pub edges_pre_solve: Vec<Edge>,
    pub facts: HashMap<usize, bool>,
}

fn _format_edges(puzzle: &Puzzle, edges: &Vec<Edge>) -> String {
    let mut res = String::new();
    for i in 0..puzzle.xsize {
        // top edges
        for j in 0..puzzle.ysize {
            let ix = puzzle.edge_ix(i, j, true);
            match edges[ix] {
                Edge::Filled => res.push_str(" -"),
                Edge::Empty => res.push_str(" x"),
                _ => res.push_str("  "),
            }
        }
        res.push_str(" \n");

        // vertical edges
        for j in 0..puzzle.ysize {
            let ix = puzzle.edge_ix(i, j, false);
            match edges[ix] {
                Edge::Filled => res.push_str("|"),
                Edge::Empty => res.push_str("x"),
                _ => res.push_str(" "),
            }

            if puzzle.cells[i][j] >= 0 {
                res.push_str(format!("{}", puzzle.cells[i][j]).as_str());
            } else {
                res.push_str(" ");
            }
        }

        match edges[puzzle.edge_ix(i, puzzle.ysize, false)] {
            Edge::Filled => res.push_str("|"),
            Edge::Empty => res.push_str("x"),
            _ => res.push_str(" "),
        }

        res.push_str("\n");
    }

    for j in 0..puzzle.ysize {
        match edges[puzzle.edge_ix(puzzle.xsize, j, true)] {
            Edge::Filled => res.push_str(" -"),
            Edge::Empty => res.push_str(" x"),
            _ => res.push_str("  "),
        }
    }
    res.push_str(" \n");

    res
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s0 = String::new();
        s0.push_str(_format_edges(&self.puzzle, &self.edges_pre_solve).as_str());
        s0.push_str("after solve:\n");
        s0.push_str(_format_edges(&self.puzzle, &self.edges).as_str());
        write!(f, "{}", s0.as_str())
    }
}

fn loop_two(a: Lit, b: Lit) -> Vec<Vec<Lit>> {
    vec![vec![a, !b], vec![!a, b]]
}
fn loop_three(a: Lit, b: Lit, c: Lit) -> Vec<Vec<Lit>> {
    // TODO can this be simplified?
    vec![
        vec![!a, !b, !c],
        vec![!a, b, c],
        vec![a, !b, c],
        vec![a, b, !c],
    ]
}
fn loop_four(a: Lit, b: Lit, c: Lit, d: Lit) -> Vec<Vec<Lit>> {
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

fn clause_zero(edges: (Lit, Lit, Lit, Lit)) -> Vec<Vec<Lit>> {
    vec![
        vec![!edges.0],
        vec![!edges.1],
        vec![!edges.2],
        vec![!edges.3],
    ]
}

fn clause_one(edges: (Lit, Lit, Lit, Lit)) -> Vec<Vec<Lit>> {
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

fn clause_two(edges: (Lit, Lit, Lit, Lit)) -> Vec<Vec<Lit>> {
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

fn clause_three(edges: (Lit, Lit, Lit, Lit)) -> Vec<Vec<Lit>> {
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
            if es.iter().all(|&l| facts.contains_key(&l.index())) {
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
                formula.add_clause(&c);
            }
        }
    }
}

fn single_loop(puzzle: &Puzzle, edges: &Vec<Edge>) -> bool {
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
            if edges[n] == Edge::Filled && !visited_edges.contains(&n) {
                queue.push_back(n);
            }
        }
    }

    all_edge_indices.sub(&visited_edges).is_empty()
}

pub fn solve(grid: Vec<Vec<Cell>>) -> Option<Vec<Solution>> {
    let xsize = grid.len();
    let ysize = grid[0].len();
    // let horizontals = (1 + xsize) * ysize;
    // let verticals = xsize * (1 + ysize);

    let p = Puzzle {
        cells: grid,
        xsize: xsize,
        ysize: ysize,
    };

    let facts = find_facts(&p);
    // let facts = HashMap::new();
    let mut base_edges = vec![Edge::Unknown; (1 + xsize) * ysize + (1 + ysize) * xsize];
    for (&k, &v) in facts.iter() {
        base_edges[k] = if v { Edge::Filled } else { Edge::Empty };
    }

    println!("simplify:\n{}", _format_edges(&p, &base_edges));

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
        println!("shiiiit son, well here's last thing:");
        match last_solution {
            Some(s) => sols.push(s),
            None => println!("oh well"),
        };
    }
    Some(sols)
}
