mod adapter;
mod data;
mod parse;
mod patterns;
mod solve_common;
mod solve_splr;
mod solve_varisat;
mod solve_z3;

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
use solve_splr::solve_splr;
use solve_varisat::solve;
use solve_z3::solve_z3;

use crate::solve_common::single_loop_edge;

const TIMEOUT_SECS: u64 = 180;
const DEFAULT_PUZZLE: &str =
// "15x15:b3a11222c3b3b1111b11b22e312a3b31d32c22g22a22c3a1a12b1b3a11b32a3e1a212c23a31a2a2a2b21221a210b22d3e0a331a22b1b1a3b12b2a32a3d3b2e1b3a222b2a23b3b21c3a2c22d3a1c21";
//    "10x10d2:3a2223a32b211a3c3a1a12a23c2b3d33c02b2c20a21a1a3b1a12a112a3e221a3k2c2a";
// "10x10d0:b1a2a22a32a1b22b2b2a23b212a22d222a31b2c12c2d331d013e1a2c2122a1b2a3b13a02a";
// "2x2:0002";
// "2x2:32";
// "10x10d2:3a2223a32b211a3c3a1a12a23c2b3d33c02b2c20a21a1a3b1a12a112a3e221a3k2c2a", ;
// "10x10d0:b1a2a22a32a1b22b2b2a23b212a22d222a31b2c12c2d331d013e1a2c2122a1b2a3b13a02a";
// "20x20d0:3b2a2a1121f12c1222a0b2212a3d02b2b2a0f0212a2a23a11d31232a2231311b3a122c12i1b22d22a2b1320b23d2a221123a2d12a0a30212b13d2a3c1a223c3c3112i1a2c230c1c332b123a3c1b3a3e2b2b31c122223h22a213b31c3a2233a1a2a1a3a3c1b3a12a1a0c2c3222a2a2d3e11a0g2b2d11e121a33b1201b22032a3a3a13a3c32b11a22e";
// "30x30d0:c333b3a1a2b3b2g2a1e0a2b102c213c3b121a11a220a3d0a21e32a2c11a22b1a1f1c2221a123b1a3231c2a2a22121a20c201c21a3b2b2a3a1a230c3a12a3b3a2a232a3a2c2c1b2b3a31a2a32d1a2a23a02a2d3a3d3b0a233b0d2b1c1a2b21b2b3b21c2b2b2b2122b01a322a2b2a1112a1b33a1b3b02a32c1a22b13a213221a121a123c01a32a21a22222c23c0a222a3a12a2b232a2f1b3b3a22a3b21a3h2b32b11a2c2a1b212c21b21f2f2b3c223a233d223a112c22221l2a20d11a21023a3b2a2120a32e2c3f2i1a3b12d2e33a22223b232a1a222a3c1b22i332b2132323a232b3a33a2d21a1a3b2a1a1b2c2f3a0c212f21b320a11a233a1a23a3c3a3a1a21a01a2a21a22i3a2c2g11f13c3a1222a11d233b3c2d22a3d3c21c12c1222b11212a32c2a11a23b2d1a13f1a110b3b3a3a2321b3213a33a3a2a2a1a";
"30x30d0:d233a2a22e3a11a3a3a3c2a3e21d221b1e212a12210c23a202a331b2e2b012a1a3a32a2a20a1a2e23f2a2b2c1021d2a02312c3a2b2a2b21b2a3b3a02b1b1023231b3c2b21c2a2c222a1a2e322a22d112a232111a113b22f1a1g0a1b1a21g2c2b3321d0c11a2b2b1a31b132d3e1g22a12d2a12d03a1a3b3112a2c2a2a2132d1132a2a12a1c322a3a322d02c2c2b1120a3a2b1b2a2b2a3c21223201a23233c22222a2a12a3e2b2212b1a33d02a3b2312b223d2a3a1b1113b2e1c00a2g1111a3a212b111d2a32a222a3f1a223a2b13b2d3c1d2a21a22a0011b12b1212a32d21j2a3c22a2b2b0a3a22b2111a21231a0322c31e12a2b3a3a3a2m2c1a21a223a3a12b23c1a21a2b3a12a32a221a1b2c33b321a3b211a2d2d3113a1a3a111d1c231a1a3b2c21b2a1a122c31a2a23a3b2h1d2b3d2b22b1b2a33221a332a2b";

// (solver_label, pre_solve, solutions, elapsed)
type SolveResult = (&'static str, bool, Vec<Solution>, Duration);

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

    let (tx, rx) = mpsc::channel::<SolveResult>();
    // Dedicated channel for pre_solve edges from the varisat-pre thread.
    let (pre_edges_tx, pre_edges_rx) = mpsc::channel::<Vec<Edge>>();

    // Thread 1: varisat, no pre-solve
    {
        let tx = tx.clone();
        let grid = grid.clone();
        thread::spawn(move || {
            let prefix = "[varisat / no-pre] ";
            println!("{prefix}Starting...");
            let t0 = Instant::now();
            if let Some(sols) = solve(grid, false, prefix) {
                tx.send((prefix, false, sols, t0.elapsed())).ok();
            }
        });
    }

    // Thread 2: varisat, pre-solve (also sends pre_edges for comparison section)
    {
        let tx = tx.clone();
        let grid = grid.clone();
        let xsize = grid.len();
        let ysize = grid[0].len();
        thread::spawn(move || {
            let prefix = "[varisat / pre   ] ";
            println!("{prefix}Starting...");
            let t0 = Instant::now();
            let p = Puzzle {
                cells: grid.clone(),
                xsize,
                ysize,
            };
            let facts = find_facts(&p);
            let total = (1 + xsize) * ysize + (1 + ysize) * xsize;
            let mut pre_edges = vec![Edge::Unknown; total];
            for (&k, &v) in &facts {
                pre_edges[k] = if v { Edge::Filled } else { Edge::Empty };
            }
            pre_edges_tx.send(pre_edges).ok();
            if let Some(sols) = solve(grid, true, prefix) {
                tx.send((prefix, true, sols, t0.elapsed())).ok();
            }
        });
    }

    // Thread 3: splr, no pre-solve
    {
        let tx = tx.clone();
        let grid = grid.clone();
        thread::spawn(move || {
            let prefix = "[splr    / no-pre] ";
            println!("{prefix}Starting...");
            let t0 = Instant::now();
            if let Some(sols) = solve_splr(grid, false, prefix) {
                tx.send((prefix, false, sols, t0.elapsed())).ok();
            }
        });
    }

    // Thread 4: splr, pre-solve
    {
        let tx = tx.clone();
        let grid = grid.clone();
        thread::spawn(move || {
            let prefix = "[splr    / pre   ] ";
            println!("{prefix}Starting...");
            let t0 = Instant::now();
            if let Some(sols) = solve_splr(grid, true, prefix) {
                tx.send((prefix, true, sols, t0.elapsed())).ok();
            }
        });
    }

    // Thread 5: z3, no pre-solve
    {
        let tx = tx.clone();
        let grid = grid.clone();
        thread::spawn(move || {
            let prefix = "[z3      / no-pre] ";
            println!("{prefix}Starting...");
            let t0 = Instant::now();
            if let Some(sols) = solve_z3(grid, false, prefix) {
                tx.send((prefix, false, sols, t0.elapsed())).ok();
            }
        });
    }

    // Thread 6: z3, pre-solve
    {
        let tx = tx.clone();
        let grid = grid.clone();
        thread::spawn(move || {
            let prefix = "[z3      / pre   ] ";
            println!("{prefix}Starting...");
            let t0 = Instant::now();
            if let Some(sols) = solve_z3(grid, true, prefix) {
                tx.send((prefix, true, sols, t0.elapsed())).ok();
            }
        });
    }
    drop(tx);

    // Collect pre_solve edges — available almost instantly.
    let pre_solve_edges = pre_edges_rx.recv_timeout(Duration::from_secs(10)).ok();

    let deadline = Instant::now() + Duration::from_secs(TIMEOUT_SECS);
    // (solver_label, pre_solve) -> (solutions, elapsed)
    let mut results: HashMap<(&'static str, bool), (Vec<Solution>, Duration)> = HashMap::new();

    for _ in 0..6 {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            println!("Timeout reached, proceeding with available results.");
            break;
        }
        match rx.recv_timeout(remaining) {
            Ok((label, pre, sols, elapsed)) => {
                println!("{label}Completed: {} solution(s).", sols.len());
                results.insert((label, pre), (sols, elapsed));
            }
            Err(_) => {
                println!("Timeout reached, proceeding with available results.");
                break;
            }
        }
    }

    // Helper to look up by solver/pre combination
    let varisat_no_pre = "[varisat / no-pre] ";
    let varisat_pre = "[varisat / pre   ] ";
    let splr_no_pre = "[splr    / no-pre] ";
    let splr_pre = "[splr    / pre   ] ";
    let z3_no_pre = "[z3      / no-pre] ";
    let z3_pre = "[z3      / pre   ] ";

    let get_time = |label: &'static str, pre: bool| -> String {
        match results.get(&(label, pre)) {
            Some((_, d)) => format!("{:.2?}", d),
            None => "timed out".to_string(),
        }
    };

    'analysis: {
        // For the comparison section use varisat results as reference.
        let sols_no_pre = results.get(&(varisat_no_pre, false)).map(|(s, _)| s);
        let sols_pre = results.get(&(varisat_pre, true)).map(|(s, _)| s);

        println!("\n=== Result ===\n");

        // Primary display: varisat/pre shows pre-solve deductions and final solution together.
        // Fall back to varisat/no-pre if not available.
        let primary = sols_pre
            .and_then(|v| v.first())
            .or_else(|| sols_no_pre.and_then(|v| v.first()));
        match primary {
            Some(sol) => print!("{}", sol),
            None => println!("No solution found (solvers timed out or found no solutions)."),
        }

        // Reference edges (varisat/no-pre) for comparing other solvers.
        let ref_edges = sols_no_pre.and_then(|v| v.first()).map(|s| &s.edges);

        // Print other solver results only when they differ from the reference.
        for &(label, pre) in &[
            (splr_no_pre, false),
            (splr_pre, true),
            (z3_no_pre, false),
            (z3_pre, true),
        ] {
            match results.get(&(label, pre)) {
                Some((sols, _)) if !sols.is_empty() => {
                    if ref_edges.is_none_or(|r| *r != sols[0].edges) {
                        println!("\n{} differs from reference:\n{}", label.trim(), sols[0]);
                    }
                }
                Some(_) if ref_edges.is_some() => {
                    println!(
                        "{} found no solutions (reference has a solution).",
                        label.trim()
                    );
                }
                _ => {}
            }
        }

        // Ground truth: first valid single-loop from varisat / no-pre.
        let sol_false = match sols_no_pre.and_then(|v| v.first()) {
            // TODO: add this check back in: commented bc I expect solution vector here now instead
            Some(s) if single_loop_edge(&s.puzzle, &s.edges) => s,
            _ => {
                println!("Cannot compare: varisat/no-pre did not produce a valid loop.");
                break 'analysis;
            }
        };
        let puzzle = &sol_false.puzzle;
        let col_width = 2 * puzzle.ysize + 1;

        // Determine whether varisat / pre-solve produced a valid loop.
        let pre_true_has_loop = sols_pre
            .and_then(|v| v.first())
            .is_some_and(|s| single_loop_edge(&s.puzzle, &s.edges));

        if pre_true_has_loop {
            let sol_true = sols_pre.unwrap().first().unwrap();

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
            // varisat/pre reached a dead end (UNSAT, non-loop fallback, or timed out).
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
                            break 'analysis;
                        }
                        (w, 0) => format!("{w} wrong (red)"),
                        (0, m) => format!("{m} missing (yellow bg)"),
                        (w, m) => format!("{w} wrong (red), {m} missing (yellow bg)"),
                    };
                    println!(
                        "\n=== Dead-end analysis: pre_solve vs ground truth — {summary} ===\n"
                    );
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
    } // end 'analysis

    println!("\n=== Timing Matrix ===");
    println!(
        "  {:<22} | {:<12} | {:<12}",
        "", "no pre-solve", "with pre-solve"
    );
    println!(
        "  {:<22} | {:<12} | {:<12}",
        "varisat",
        get_time(varisat_no_pre, false),
        get_time(varisat_pre, true)
    );
    println!(
        "  {:<22} | {:<12} | {:<12}",
        "splr",
        get_time(splr_no_pre, false),
        get_time(splr_pre, true)
    );
    println!(
        "  {:<22} | {:<12} | {:<12}",
        "z3",
        get_time(z3_no_pre, false),
        get_time(z3_pre, true)
    );
}
