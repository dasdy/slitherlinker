pub mod parse;
pub mod solve;
pub mod puzzle;
use parse::from_string;
use solve::solve;

pub fn main() {
    // let g = from_string("2x2:0002").unwrap();
    // let g = from_string("2x2:32").unwrap();
    let g = from_string("10x10:3a2223a32b211a3c3a1a12a23c2b3d33c02b2c20a21a1a3b1a12a112a3e221a3k2c2a").unwrap();
    
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
