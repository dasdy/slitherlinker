pub mod parse;
pub mod solve;
pub mod puzzle;
pub mod patterns;
use parse::from_string;
use solve::solve;
use puzzle::Puzzle;
use patterns::find_facts;
use std::env;

use crate::{puzzle::Edge, solve::Solution};

pub fn main() {
    // let g = from_string("2x2:0002").unwrap();
    // let g = from_string("2x2:32").unwrap();
    // let g = from_string("10x10:3a2223a32b211a3c3a1a12a23c2b3d33c02b2c20a21a1a3b1a12a112a3e221a3k2c2a").unwrap();
    // let g = from_string("10x10d2:3a2223a32b211a3c3a1a12a23c2b3d33c02b2c20a21a1a3b1a12a112a3e221a3k2c2a").unwrap();
    // let g = from_string("10x10d0:b1a2a22a32a1b22b2b2a23b212a22d222a31b2c12c2d331d013e1a2c2122a1b2a3b13a02a").unwrap();
    let g = from_string("20x20d0:3b2a2a1121f12c1222a0b2212a3d02b2b2a0f0212a2a23a11d31232a2231311b3a122c12i1b22d22a2b1320b23d2a221123a2d12a0a30212b13d2a3c1a223c3c3112i1a2c230c1c332b123a3c1b3a3e2b2b31c122223h22a213b31c3a2233a1a2a1a3a3c1b3a12a1a0c2c3222a2a2d3e11a0g2b2d11e121a33b1201b22032a3a3a13a3c32b11a22e").unwrap();
    // let g = from_string("30x30d0:c333b3a1a2b3b2g2a1e0a2b102c213c3b121a11a220a3d0a21e32a2c11a22b1a1f1c2221a123b1a3231c2a2a22121a20c201c21a3b2b2a3a1a230c3a12a3b3a2a232a3a2c2c1b2b3a31a2a32d1a2a23a02a2d3a3d3b0a233b0d2b1c1a2b21b2b3b21c2b2b2b2122b01a322a2b2a1112a1b33a1b3b02a32c1a22b13a213221a121a123c01a32a21a22222c23c0a222a3a12a2b232a2f1b3b3a22a3b21a3h2b32b11a2c2a1b212c21b21f2f2b3c223a233d223a112c22221l2a20d11a21023a3b2a2120a32e2c3f2i1a3b12d2e33a22223b232a1a222a3c1b22i332b2132323a232b3a33a2d21a1a3b2a1a1b2c2f3a0c212f21b320a11a233a1a23a3c3a3a1a21a01a2a21a22i3a2c2g11f13c3a1222a11d233b3c2d22a3d3c21c12c1222b11212a32c2a11a23b2d1a13f1a110b3b3a3a2321b3213a33a3a2a2a1a").unwrap();
    // let args: Vec<String> = env::args().collect();

    // let puzzle = &args[1];
    // println!("puzzle: {puzzle}");
    // let g = from_string(&puzzle).unwrap();

    let xsize = g.len();
    let ysize = g[0].len();
    // let horizontals = (1 + xsize) * ysize;
    // let verticals = xsize * (1 + ysize);

    // let p = Puzzle {
    //     cells: g.clone(),
    //     xsize: xsize,
    //     ysize: ysize,
    // };

    let maybe_solution = solve(g);
    match maybe_solution {
        Some(sols) => {
            for (i, s) in sols.iter().enumerate() {
                println!("{i}:\n{s}")
            }
        },
        None => println!("eh")
    }
}
