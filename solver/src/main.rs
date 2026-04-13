mod adapter;
mod data;
mod parse;
mod patterns;
mod solve_common;
mod solve_splr;
mod solve_varisat;

use std::collections::HashMap;
use std::env;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use data::pattern::Edge;
use data::puzzle::Puzzle;
use data::solution::{format_puzzle_diff, format_side_by_side, Solution, ANSI_RED, ANSI_YELLOW_BG};
use parse::from_string;
use patterns::find_facts;
use solve_common::single_loop;
use solve_varisat::solve;

const TIMEOUT_SECS: u64 = 180;
const DEFAULT_PUZZLE: &str =
"15x15:b3a11222c3b3b1111b11b22e312a3b31d32c22g22a22c3a1a12b1b3a11b32a3e1a212c23a31a2a2a2b21221a210b22d3e0a331a22b1b1a3b12b2a32a3d3b2e1b3a222b2a23b3b21c3a2c22d3a1c21";
// "10x10d2:3a2223a32b211a3c3a1a12a23c2b3d33c02b2c20a21a1a3b1a12a112a3e221a3k2c2a";
// "10x10d0:b1a2a22a32a1b22b2b2a23b212a22d222a31b2c12c2d331d013e1a2c2122a1b2a3b13a02a";
// "2x2:0002";
// "2x2:32";
// "10x10d2:3a2223a32b211a3c3a1a12a23c2b3d33c02b2c20a21a1a3b1a12a112a3e221a3k2c2a", ;
// "10x10d0:b1a2a22a32a1b22b2b2a23b212a22d222a31b2c12c2d331d013e1a2c2122a1b2a3b13a02a";
// "20x20d0:3b2a2a1121f12c1222a0b2212a3d02b2b2a0f0212a2a23a11d31232a2231311b3a122c12i1b22d22a2b1320b23d2a221123a2d12a0a30212b13d2a3c1a223c3c3112i1a2c230c1c332b123a3c1b3a3e2b2b31c122223h22a213b31c3a2233a1a2a1a3a3c1b3a12a1a0c2c3222a2a2d3e11a0g2b2d11e121a33b1201b22032a3a3a13a3c32b11a22e";
// "30x30d0:c333b3a1a2b3b2g2a1e0a2b102c213c3b121a11a220a3d0a21e32a2c11a22b1a1f1c2221a123b1a3231c2a2a22121a20c201c21a3b2b2a3a1a230c3a12a3b3a2a232a3a2c2c1b2b3a31a2a32d1a2a23a02a2d3a3d3b0a233b0d2b1c1a2b21b2b3b21c2b2b2b2122b01a322a2b2a1112a1b33a1b3b02a32c1a22b13a213221a121a123c01a32a21a22222c23c0a222a3a12a2b232a2f1b3b3a22a3b21a3h2b32b11a2c2a1b212c21b21f2f2b3c223a233d223a112c22221l2a20d11a21023a3b2a2120a32e2c3f2i1a3b12d2e33a22223b232a1a222a3c1b22i332b2132323a232b3a33a2d21a1a3b2a1a1b2c2f3a0c212f21b320a11a233a1a23a3c3a3a1a21a01a2a21a22i3a2c2g11f13c3a1222a11d233b3c2d22a3d3c21c12c1222b11212a32c2a11a23b2d1a13f1a110b3b3a3a2321b3213a33a3a2a2a1a";

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

    // Channel for final solutions from both threads: (pre_solve, solutions, elapsed).
    let (tx, rx) = mpsc::channel::<(bool, Vec<Solution>, Duration)>();
    // Dedicated channel for pre_solve edges: sent before SAT starts so they
    // are available even if the full solve times out or reaches UNSAT.
    let (pre_edges_tx, pre_edges_rx) = mpsc::channel::<Vec<Edge>>();

    let tx1 = tx.clone();
    let grid1 = grid.clone();
    thread::spawn(move || {
        let prefix = "[SAT-only] ";
        println!("{prefix}Starting...");
        let t0 = Instant::now();
        if let Some(sols) = solve(grid1, false, prefix) {
            tx1.send((false, sols, t0.elapsed())).ok();
        }
    });

    let tx2 = tx.clone();
    let grid2 = grid.clone();
    let xsize = grid2.len();
    let ysize = grid2[0].len();
    thread::spawn(move || {
        let prefix = "[pre-solve] ";
        println!("{prefix}Starting...");
        let t0 = Instant::now();
        // Compute and send pre_solve edges immediately (find_facts is sub-second).
        let p = Puzzle {
            cells: grid2.clone(),
            xsize,
            ysize,
        };
        let facts = find_facts(&p, prefix);
        let total = (1 + xsize) * ysize + (1 + ysize) * xsize;
        let mut pre_edges = vec![Edge::Unknown; total];
        for (&k, &v) in &facts {
            pre_edges[k] = if v { Edge::Filled } else { Edge::Empty };
        }
        pre_edges_tx.send(pre_edges).ok();
        // Now run the full SAT solve.
        if let Some(sols) = solve(grid2, true, prefix) {
            tx2.send((true, sols, t0.elapsed())).ok();
        }
    });
    drop(tx);

    // Collect pre_solve edges — available almost instantly.
    let pre_solve_edges = pre_edges_rx.recv_timeout(Duration::from_secs(10)).ok();

    let deadline = Instant::now() + Duration::from_secs(TIMEOUT_SECS);
    let mut sols_no_pre: Option<Vec<Solution>> = None;
    let mut sols_pre: Option<Vec<Solution>> = None;
    let mut time_no_pre: Option<Duration> = None;
    let mut time_pre: Option<Duration> = None;

    for _ in 0..2 {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            println!("Timeout reached, proceeding with available results.");
            break;
        }
        match rx.recv_timeout(remaining) {
            Ok((false, sols, elapsed)) => {
                println!("[SAT-only] Completed: {} solution(s).", sols.len());
                time_no_pre = Some(elapsed);
                sols_no_pre = Some(sols);
            }
            Ok((true, sols, elapsed)) => {
                println!("[pre-solve] Completed: {} solution(s).", sols.len());
                time_pre = Some(elapsed);
                sols_pre = Some(sols);
            }
            Err(_) => {
                println!("Timeout reached, proceeding with available results.");
                break;
            }
        }
    }

    println!("\n=== Timing ===");
    match time_no_pre {
        Some(d) => println!("  [SAT-only]  {:.2?}", d),
        None     => println!("  [SAT-only]  timed out"),
    }
    match time_pre {
        Some(d) => println!("  [pre-solve] {:.2?}", d),
        None     => println!("  [pre-solve] timed out"),
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
        .is_some_and(|s| single_loop(&s.puzzle, &s.edges));

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
                format_side_by_side(
                    &left,
                    &right,
                    "SAT only (reference)",
                    "with pre_solve",
                    col_width
                )
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
                    format_side_by_side(
                        &left,
                        &right,
                        "Ground truth (SAT only)",
                        "Pre-solve output",
                        col_width
                    )
                );
            }
        }
    }
}
