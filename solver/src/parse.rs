use regex::Regex;
use std::error::Error;
use std::fmt;

pub type Cell = i8;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "String did not match expected pattern")
    }
}

impl Error for ParseError {}

fn parse_str(inp: &str) -> Result<(usize, usize, String)> {
    let r = Regex::new(r"^(\d+)x(\d+)(d\d+)?:(.+)$").unwrap();
    let c_opt = r.captures_iter(inp).next(); 
    match c_opt {
        Some(c) => {
            let xs = c[1].parse::<usize>();
            let ys = c[2].parse::<usize>();
            let s = String::from(&c[4]);
            Ok((xs?, ys?, s))
        },
        None => Err(Box::new(ParseError {}))
    }
}

pub fn from_string(inp: &str) -> Result<Vec<Vec<Cell>>> {
    let (xsize, ysize, encoded) = parse_str(inp)?;
    let init: Cell = -1;
    let mut res = vec![vec![init; ysize]; xsize];

    let mut ix: usize = 0;
    for c in encoded.chars() {
        if ('0'..='4').contains(&c) {
            let i: usize = ix / ysize;
            let j: usize = ix % ysize;
            res[i][j] = char::to_digit(c, 10).unwrap() as i8;
        }
        if c.is_ascii_lowercase() {
            let to_skip = ((c as i32) - ('a' as i32)) as usize;
            ix += to_skip;
        }
        ix += 1;
    }
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::from_string;
    #[test]
    fn simple_zero_case() {
        let result = from_string("2x2:0000").unwrap();
        let expected = [[0, 0],[0, 0]];
        assert!(result == expected)
    }

    #[test]
    fn real_case() {
        let result = from_string(
            "10x10:3a2223a32b211a3c3a1a12a23c2b3d33c02b2c20a21a1a3b1a12a112a3e221a3k2c2a",
        ).unwrap();
        let expected = [
            [3, -1, 2, 2, 2, 3, -1, 3, 2, -1],
            [-1, 2, 1, 1, -1, 3, -1, -1, -1, 3],
            [-1, 1, -1, 1, 2, -1, 2, 3, -1, -1],
            [-1, 2, -1, -1, 3, -1, -1, -1, -1, 3],
            [3, -1, -1, -1, 0, 2, -1, -1, 2, -1],
            [-1, -1, 2, 0, -1, 2, 1, -1, 1, -1],
            [3, -1, -1, 1, -1, 1, 2, -1, 1, 1],
            [2, -1, 3, -1, -1, -1, -1, -1, 2, 2],
            [1, -1, 3, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, 2, -1, -1, -1, 2, -1],
        ];
        assert!(result == expected);
    }
    #[test]
    fn initial_negative() {
        let result = from_string("2x2:a").unwrap();
        let expected = vec![vec![-1; 2]; 2];
        assert!(result == expected)
    }

    #[test]
    fn initial_uses_numbers() {
        let result = from_string("2x2:1234").unwrap();
        let expected = [[1, 2],[3, 4]];
        assert!(result == expected)
    }

    #[test]
    fn skips_rows() {
        let result = from_string("2x2:c4").unwrap();
        let expected = [[-1, -1], [-1, 4]];
        assert!(result == expected)
    }

    #[test]
    fn returns_error_on_bad_string(){
        assert!(from_string("2x2:").is_err());
    }

    #[test]
    fn adapts_size() {
        let result = from_string("3x5:1b2c3d4").unwrap();
        let expected = [
            [1, -1, -1, 2, -1],
            [-1, -1, 3, -1, -1],
            [-1, -1, 4, -1, -1],
        ];
        assert!(result == expected)
    }
}
