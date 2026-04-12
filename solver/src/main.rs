mod data;
mod parse;
mod patterns;
mod solve_common;
mod solve_varisat;
mod solve_splr;
mod adapter;

use std::collections::HashMap;
use std::env;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use parse::from_string;
use solve_varisat::solve;
use solve_common::single_loop;
use patterns::find_facts;
use data::pattern::Edge;
use data::puzzle::Puzzle;
use data::solution::{ANSI_RED, ANSI_YELLOW_BG, format_puzzle_diff, format_side_by_side, Solution};

const TIMEOUT_SECS: u64 = 180;
const DEFAULT_PUZZLE: &str =
    "10x10d0:b1a2a22a32a1b22b2b2a23b212a22d222a31b2c12c2d331d013e1a2c2122a1b2a3b13a02a";

pub fn main() {
    let puzzle_str = {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            args[1].clone()
        } else {
            DEFAULT_PUZZLE.to_string()
        }
    };

    println!("Puzzle: {puzzle_str}\n");

    let grid = from_string(&puzzle_str).unwrap();

    // Channel for final solutions from both threads.
    let (tx, rx) = mpsc::channel::<(bool, Vec<Solution>)>();
    // Dedicated channel for pre_solve edges: sent before SAT starts so they
    // are available even if the full solve times out or reaches UNSAT.
    let (pre_edges_tx, pre_edges_rx) = mpsc::channel::<Vec<Edge>>();

    let tx1 = tx.clone();
    let grid1 = grid.clone();
    thread::spawn(move || {
        println!("[pre_solve=false] Starting...");
        if let Some(sols) = solve(grid1, false) {
            tx1.send((false, sols)).ok();
        }
    });

    let tx2 = tx.clone();
    let grid2 = grid.clone();
    let xsize = grid2.len();
    let ysize = grid2[0].len();
    thread::spawn(move || {
        println!("[pre_solve=true] Starting...");
        // Compute and send pre_solve edges immediately (find_facts is sub-second).
        let p = Puzzle { cells: grid2.clone(), xsize, ysize };
        let facts = find_facts(&p);
        let total = (1 + xsize) * ysize + (1 + ysize) * xsize;
        let mut pre_edges = vec![Edge::Unknown; total];
        for (&k, &v) in &facts {
            pre_edges[k] = if v { Edge::Filled } else { Edge::Empty };
        }
        pre_edges_tx.send(pre_edges).ok();
        // Now run the full SAT solve.
        if let Some(sols) = solve(grid2, true) {
            tx2.send((true, sols)).ok();
        }
    });
    drop(tx);

    // Collect pre_solve edges — available almost instantly.
    let pre_solve_edges = pre_edges_rx.recv_timeout(Duration::from_secs(10)).ok();

    let deadline = Instant::now() + Duration::from_secs(TIMEOUT_SECS);
    let mut sols_no_pre: Option<Vec<Solution>> = None;
    let mut sols_pre: Option<Vec<Solution>> = None;

    for _ in 0..2 {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            println!("Timeout reached, proceeding with available results.");
            break;
        }
        match rx.recv_timeout(remaining) {
            Ok((false, sols)) => {
                println!("[pre_solve=false] Completed: {} solution(s).", sols.len());
                sols_no_pre = Some(sols);
            }
            Ok((true, sols)) => {
                println!("[pre_solve=true] Completed: {} solution(s).", sols.len());
                sols_pre = Some(sols);
            }
            Err(_) => {
                println!("Timeout reached, proceeding with available results.");
                break;
            }
        }
    }

    println!("\n=== Individual results ===\n");

    match &sols_no_pre {
        Some(sols) if !sols.is_empty() => println!("pre_solve=false:\n{}", sols[0]),
        Some(_) => println!("pre_solve=false: no solutions found."),
        None => println!("pre_solve=false: timed out."),
    }

    match &sols_pre {
        Some(sols) if !sols.is_empty() => println!("pre_solve=true:\n{}", sols[0]),
        Some(_) => println!("pre_solve=true: no solutions found."),
        None => println!("pre_solve=true: timed out."),
    }

    // Ground truth: first valid single-loop from pre_solve=false.
    let sol_false = match sols_no_pre.as_ref().and_then(|v| v.first()) {
        Some(s) if single_loop(&s.puzzle, &s.edges) => s,
        _ => {
            println!("Cannot compare: pre_solve=false did not produce a valid loop.");
            return;
        }
    };
    let puzzle = &sol_false.puzzle;
    let col_width = 2 * puzzle.ysize + 1;

    // Determine whether pre_solve=true produced a valid loop.
    let pre_true_has_loop = sols_pre
        .as_ref()
        .and_then(|v| v.first())
        .map_or(false, |s| single_loop(&s.puzzle, &s.edges));

    if pre_true_has_loop {
        let sol_true = sols_pre.as_ref().unwrap().first().unwrap();

        // Level A: compare final SAT solutions between the two runs.
        let highlights_a: HashMap<usize, &'static str> = sol_false
            .edges
            .iter()
            .enumerate()
            .filter(|(i, e)| sol_true.edges.get(*i) != Some(e))
            .map(|(i, _)| (i, ANSI_RED))
            .collect();

        if highlights_a.is_empty() {
            println!("\n=== Level A: Final solutions are identical. ===\n");
        } else {
            println!(
                "\n=== Level A: Final solutions differ on {} edge(s) (red = mismatch) ===\n",
                highlights_a.len()
            );
            let left = format_puzzle_diff(puzzle, &sol_false.edges, &highlights_a);
            let right = format_puzzle_diff(puzzle, &sol_true.edges, &highlights_a);
            print!(
                "{}",
                format_side_by_side(&left, &right, "SAT only (reference)", "with pre_solve", col_width)
            );
        }

        // Level B: compare pre_solve deductions against the ground-truth final solution.
        let highlights_b: HashMap<usize, &'static str> = sol_true
            .edges_pre_solve
            .iter()
            .enumerate()
            .filter(|(_, &e)| e != Edge::Unknown)
            .filter(|(i, &e)| sol_false.edges.get(*i) != Some(&e))
            .map(|(i, _)| (i, ANSI_RED))
            .collect();

        if highlights_b.is_empty() {
            println!("\n=== Level B: All pre_solve deductions match the ground truth. ===\n");
        } else {
            println!(
                "\n=== Level B: pre_solve made {} wrong deduction(s) vs ground truth (red = wrong) ===\n",
                highlights_b.len()
            );
            let left = format_puzzle_diff(puzzle, &sol_false.edges, &highlights_b);
            let right = format_puzzle_diff(puzzle, &sol_true.edges_pre_solve, &highlights_b);
            print!(
                "{}",
                format_side_by_side(
                    &left,
                    &right,
                    "Ground truth (SAT only)",
                    "Pre-solve deductions",
                    col_width
                )
            );
        }
    } else {
        // pre_solve=true reached a dead end (UNSAT, non-loop fallback, or timed out).
        // Compare pre_solve deductions against ground truth with two-color highlighting:
        //   red            = wrong deduction (pre_solve asserted the wrong state)
        //   yellow bg      = missing deduction (ground truth Filled, pre_solve Unknown)
        match &pre_solve_edges {
            None => println!("\nCannot compare: pre_solve edges not available."),
            Some(pre_edges) => {
                let mut highlights_left: HashMap<usize, &'static str> = HashMap::new();
                let mut highlights_right: HashMap<usize, &'static str> = HashMap::new();
                let mut wrong_count = 0usize;
                let mut missing_count = 0usize;

                for (i, &pre_e) in pre_edges.iter().enumerate() {
                    let ground = sol_false.edges.get(i).copied().unwrap_or(Edge::Unknown);
                    if pre_e != Edge::Unknown && pre_e != ground {
                        highlights_left.insert(i, ANSI_RED);
                        highlights_right.insert(i, ANSI_RED);
                        wrong_count += 1;
                    } else if ground == Edge::Filled && pre_e == Edge::Unknown {
                        highlights_left.insert(i, ANSI_YELLOW_BG);
                        highlights_right.insert(i, ANSI_YELLOW_BG);
                        missing_count += 1;
                    }
                }

                let summary = match (wrong_count, missing_count) {
                    (0, 0) => {
                        println!("\n=== Dead-end analysis: pre_solve deductions are all correct (dead end has another cause). ===\n");
                        return;
                    }
                    (w, 0) => format!("{w} wrong (red)"),
                    (0, m) => format!("{m} missing (yellow bg)"),
                    (w, m) => format!("{w} wrong (red), {m} missing (yellow bg)"),
                };
                println!("\n=== Dead-end analysis: pre_solve vs ground truth — {summary} ===\n");
                let left = format_puzzle_diff(puzzle, &sol_false.edges, &highlights_left);
                let right = format_puzzle_diff(puzzle, pre_edges, &highlights_right);
                print!(
                    "{}",
                    format_side_by_side(&left, &right, "Ground truth (SAT only)", "Pre-solve output", col_width)
                );
            }
        }
    }
}
