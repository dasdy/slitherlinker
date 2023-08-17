use std::{
    collections::{HashSet, LinkedList},
    ops::{Not, Sub},
};

use crate::data::pattern::Edge;
use crate::data::puzzle::Puzzle;

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
    T: Not<Output = T> + Copy,
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
            if edges[n] == Edge::Filled && !visited_edges.contains(&n) {
                queue.push_back(n);
            }
        }
    }

    all_edge_indices.sub(&visited_edges).is_empty()
}
