use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Cell {
    Any,
    OutOfBounds,
    Nothing,
    Zero,
    One,
    Two,
    Three,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Edge {
    Any,
    OutOfBounds,
    Unknown,
    Empty,
    EmptyStrict,
    Filled,
}

impl Cell {
    fn matches(&self, other: &Self) -> bool {
        *self == Cell::Any || *other == Cell::Any || *self == *other
    }
}

impl Edge {
    fn matches(&self, other: &Self) -> bool {
        *self == Edge::Any
            || *other == Edge::Any
            || *self == *other
            || (*self == Edge::OutOfBounds && *other == Edge::Empty)
            || (*self == Edge::Empty && *other == Edge::OutOfBounds)
            || (*self == Edge::EmptyStrict && *other == Edge::Empty)
            || (*self == Edge::Empty && *other == Edge::EmptyStrict)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pattern {
    pub horizontals: Horizontals,
    pub verticals: Verticals,
}

pub type Horizontals = [[Edge; 3]; 2];
pub type Verticals = [[Edge; 2]; 3];
pub type CellWindow = [[Cell; 3]; 3];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PatternSolution {
    pub cells: CellWindow,
    pub input: Pattern,
    pub output: Pattern,
}

impl fmt::Display for PatternSolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        res.push_str("IN:\n");
        for i in 0..self.cells.len() {
            // verticals
            res.push_str("* ");
            for j in 0..self.cells[0].len() - 1 {
                res.push_str(match self.cells[i][j] {
                    Cell::Zero => "0",
                    Cell::One => "1",
                    Cell::Two => "2",
                    Cell::Three => "3",
                    Cell::OutOfBounds => "B",
                    _ => " ",
                });

                res.push_str(match self.input.verticals[i][j] {
                    Edge::Filled => "|",
                    Edge::Empty => "x",
                    Edge::OutOfBounds => "%",
                    _ => " ",
                })
            }
            res.push_str(match self.cells[i].last().unwrap() {
                Cell::Zero => "0",
                Cell::One => "1",
                Cell::Two => "2",
                Cell::Three => "3",
                Cell::OutOfBounds => "B",
                _ => " ",
            });

            res.push_str("*\n*");

            // horizontals
            if i < 2 {
                for j in 0..self.input.horizontals[i].len() {
                    res.push_str(match self.input.horizontals[i][j] {
                        Edge::Filled => ".-",
                        Edge::Empty => ".x",
                        Edge::OutOfBounds => ".%",
                        _ => ". ",
                    })
                }
            }
            res.push_str("*\n");
        }

        res.push_str("OUT:\n");

        for i in 0..self.cells.len() {
            res.push_str("* ");
            // verticals
            for j in 0..self.cells[0].len() - 1 {
                res.push_str(match self.cells[i][j] {
                    Cell::Zero => "0",
                    Cell::One => "1",
                    Cell::Two => "2",
                    Cell::Three => "3",
                    Cell::OutOfBounds => "B",
                    _ => " ",
                });

                res.push_str(match self.output.verticals[i][j] {
                    Edge::Filled => "|",
                    Edge::Empty => "x",
                    Edge::OutOfBounds => "%",
                    _ => " ",
                })
            }
            res.push_str(match self.cells[i].last().unwrap() {
                Cell::Zero => "0",
                Cell::One => "1",
                Cell::Two => "2",
                Cell::Three => "3",
                Cell::OutOfBounds => "B",
                _ => " ",
            });

            res.push_str("*\n");

            // horizontals
            if i < 2 {
                res.push('*');
                for j in 0..self.output.horizontals[i].len() {
                    res.push_str(match self.output.horizontals[i][j] {
                        Edge::Filled => ".-",
                        Edge::Empty => ".x",
                        Edge::OutOfBounds => "%",
                        _ => ". ",
                    })
                }
                res.push('*');
            }
            res.push('\n');
        }

        f.write_str(&res)
    }
}

pub fn rot90<T: Copy, const W: usize, const H: usize>(v: &[[T; W]; H]) -> [[T; H]; W] {
    let mut r = [[v[0][0]; H]; W];
    for i in 0..v.len() {
        for j in 0..v[0].len() {
            let e = v[i][j];
            r[j][r[0].len() - i - 1] = e;
        }
    }
    r
}

fn mirror<T: Copy, const W: usize, const H: usize>(v: &[[T; W]; H]) -> [[T; W]; H] {
    let mut r = *v;
    for i in 0..v.len() {
        for j in 0..v[0].len() {
            let e = v[i][j];
            r[i][r[0].len() - j - 1] = e;
        }
    }
    r
}

impl Pattern {
    //clokwise rotate
    pub fn rot90(&self) -> Pattern {
        let mut n = *self;
        n.verticals = rot90(&self.horizontals);
        n.horizontals = rot90(&self.verticals);
        n
    }

    fn mirror(&self) -> Pattern {
        let mut n = *self;
        n.verticals = mirror(&self.verticals);
        n.horizontals = mirror(&self.horizontals);
        n
    }
}

fn parse_cell(c: char) -> Cell {
    match c {
        '*' => Cell::Any,
        '0' => Cell::Zero,
        '1' => Cell::One,
        '2' => Cell::Two,
        '3' => Cell::Three,
        'B' => Cell::OutOfBounds,
        '.' => Cell::Nothing,
        _ => panic!("Unknown cell char: {c:?}"),
    }
}

fn parse_vert(c: char) -> Edge {
    match c {
        '*' => Edge::Any,
        '|' => Edge::Filled,
        'x' => Edge::Empty,
        'X' => Edge::EmptyStrict,
        '%' => Edge::OutOfBounds,
        '?' => Edge::Unknown,
        _ => panic!("Unknown vertical edge char: {c:?}"),
    }
}

fn parse_horiz(c: char) -> Edge {
    match c {
        '*' => Edge::Any,
        '-' => Edge::Filled,
        'x' => Edge::Empty,
        'X' => Edge::EmptyStrict,
        '%' => Edge::OutOfBounds,
        '?' => Edge::Unknown,
        _ => panic!("Unknown horizontal edge char: {c:?}"),
    }
}

fn parse_cell_row(line: &str) -> ([Cell; 3], [Edge; 2]) {
    let chars: Vec<char> = line.chars().collect();
    assert_eq!(chars.len(), 5, "Cell row must be 5 chars, got {:?}", line);
    (
        [
            parse_cell(chars[0]),
            parse_cell(chars[2]),
            parse_cell(chars[4]),
        ],
        [parse_vert(chars[1]), parse_vert(chars[3])],
    )
}

fn parse_horiz_row(line: &str) -> [Edge; 3] {
    let chars: Vec<char> = line.chars().collect();
    assert_eq!(
        chars.len(),
        5,
        "Horizontal edge row must be 5 chars, got {:?}",
        line
    );
    [
        parse_horiz(chars[0]),
        parse_horiz(chars[2]),
        parse_horiz(chars[4]),
    ]
}

/// Parse a compact pattern string into a `(CellWindow, Pattern)`.
///
/// The string must have exactly 5 non-empty lines:
/// ```text
/// c v c v c   <- cell row (5 chars: cell, vert-edge, cell, vert-edge, cell)
/// h . h . h   <- horizontal edge row (5 chars; positions 1,3 are ignored separators)
/// c v c v c
/// h . h . h
/// c v c v c   <- bottom cell row (no edge row below)
/// ```
///
/// Cell characters: `*`=Any, `0-3`=value, `B`=OutOfBounds, `.`=Nothing
/// Vertical edge chars (odd positions in cell rows): `*`=Any, `|`=Filled, `x`=Empty, `X`=EmptyStrict, `%`=OutOfBounds, `?`=Unknown
/// Horizontal edge chars (positions 0,2,4 of horiz rows): `*`=Any, `-`=Filled, `x`=Empty, `X`=EmptyStrict, `%`=OutOfBounds, `?`=Unknown
fn parse_str(s: &str) -> (CellWindow, Pattern) {
    let lines: Vec<&str> = s.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines.len(),
        5,
        "Pattern string must have 5 non-empty lines, got {}",
        lines.len()
    );
    let (c0, v0) = parse_cell_row(lines[0]);
    let h0 = parse_horiz_row(lines[1]);
    let (c1, v1) = parse_cell_row(lines[2]);
    let h1 = parse_horiz_row(lines[3]);
    let (c2, v2) = parse_cell_row(lines[4]);
    (
        [
            [c0[0], c0[1], c0[2]],
            [c1[0], c1[1], c1[2]],
            [c2[0], c2[1], c2[2]],
        ],
        Pattern {
            horizontals: [h0, h1],
            verticals: [[v0[0], v0[1]], [v1[0], v1[1]], [v2[0], v2[1]]],
        },
    )
}

impl PatternSolution {
    /// Parse a `PatternSolution` from two compact string representations.
    /// See [`parse_str`] for the string format.
    /// The `output` string's cell characters are ignored; cells are taken from `input`.
    #[allow(dead_code)]
    pub fn parse(input: &str, output: &str) -> PatternSolution {
        let (cells, input_pattern) = parse_str(input);
        let (_, output_pattern) = parse_str(output);
        PatternSolution {
            cells,
            input: input_pattern,
            output: output_pattern,
        }
    }

    fn rot90(&self) -> PatternSolution {
        PatternSolution {
            cells: rot90(&self.cells),
            input: self.input.rot90(),
            output: self.output.rot90(),
        }
    }

    fn reflect(&self) -> PatternSolution {
        PatternSolution {
            cells: mirror(&self.cells),
            input: self.input.mirror(),
            output: self.output.mirror(),
        }
    }
    pub fn rotations(&self) -> Vec<PatternSolution> {
        let mut res: Vec<PatternSolution> = vec![];
        res.push(*self);
        for _ in 0..3 {
            res.push(res.last().unwrap().rot90());
        }
        let cur_len = res.len();
        for i in 0..cur_len {
            res.push(res[i].reflect());
        }
        let cur_len = res.len();
        for i in 0..cur_len {
            res.push(res[i].rot90());
            res.push(res[i].rot90().rot90());
            res.push(res[i].rot90().rot90().rot90());

            res.push(res[i].rot90().reflect());
            res.push(res[i].rot90().rot90().reflect());
            res.push(res[i].rot90().rot90().rot90().reflect());
        }

        for i in 0..cur_len {
            res.push(res[i].rot90());
            res.push(res[i].rot90().rot90());
            res.push(res[i].rot90().rot90().rot90());

            res.push(res[i].reflect().rot90());
            res.push(res[i].reflect().rot90().rot90());
            res.push(res[i].reflect().rot90().rot90().rot90());
        }

        let h: HashSet<PatternSolution> = HashSet::from_iter(res);
        let mut r: Vec<PatternSolution> = h.iter().copied().collect();

        r.sort();
        // let r = res.iter().map(|&x| x).collect();
        r

        // let r1 = self.rot90();
        // let r2 = r1.rot90();
        // let r3 = r2.rot90();

        // let self_refl = self.reflect();
        // let r1_refl = self_refl.rot90();
        // let r2_refl = r1_refl.rot90();
        // let r3_refl = r2_refl.rot90();

        // vec![self.clone(), r1, r2, r3, self_refl, r1_refl, r2_refl, r3_refl]
    }

    pub fn try_match(
        &self,
        cells: &CellWindow,
        horizontals: &Horizontals,
        verticals: &Verticals,
    ) -> bool {
        for (i, h_row) in horizontals.iter().enumerate() {
            for (j, &h_ij) in h_row.iter().enumerate() {
                let input_edge = self.input.horizontals[i][j];
                if !input_edge.matches(&h_ij) {
                    return false;
                }
                let output_edge = self.output.horizontals[i][j];
                if (output_edge == Edge::Empty || output_edge == Edge::Filled)
                    && (h_ij == Edge::Empty || h_ij == Edge::Filled)
                    && !output_edge.matches(&h_ij)
                {
                    return false;
                }
            }
        }
        for (i, v_row) in verticals.iter().enumerate() {
            for (j, &v_ij) in v_row.iter().enumerate() {
                if !self.input.verticals[i][j].matches(&v_ij) {
                    return false;
                }
                let output_edge = self.output.verticals[i][j];
                if (output_edge == Edge::Empty || output_edge == Edge::Filled)
                    && (v_ij == Edge::Empty || v_ij == Edge::Filled)
                    && !output_edge.matches(&v_ij)
                {
                    return false;
                }
            }
        }

        for (i, c_row) in cells.iter().enumerate() {
            for (j, &c_ij) in c_row.iter().enumerate() {
                if !self.cells[i][j].matches(&c_ij) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_parse_two_threes_vertical() {
        // Two vertically adjacent 3-cells: the horizontal edge between them must be filled
        let p = PatternSolution::parse(
            "**3**\n\
             *.*.*\n\
             **3**\n\
             *.*.*\n\
             *****",
            "**3**\n\
             *.-.*\n\
             **3**\n\
             *.*.*\n\
             *****",
        );
        assert_eq!(p.cells[0][1], Cell::Three);
        assert_eq!(p.cells[1][1], Cell::Three);
        assert_eq!(p.output.horizontals[0][1], Edge::Filled);
        assert_eq!(p.output.horizontals[0][0], Edge::Any);
        assert_eq!(p.output.horizontals[1][1], Edge::Any);
        assert_eq!(p.input.horizontals, [[Edge::Any; 3]; 2]);
    }

    #[test]
    fn test_parse_vertical_filled_edge() {
        let p = PatternSolution::parse(
            "3|3**\n\
             *.*.*\n\
             *****\n\
             *.*.*\n\
             *****",
            "3|3**\n\
             *.*.*\n\
             *****\n\
             *.*.*\n\
             *****",
        );
        assert_eq!(p.cells[0][0], Cell::Three);
        assert_eq!(p.cells[0][1], Cell::Three);
        assert_eq!(p.input.verticals[0][0], Edge::Filled);
        assert_eq!(p.output.verticals[0][0], Edge::Filled);
    }

    #[test]
    fn test_rotate_array() {
        let r = rot90(&[[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
        assert_eq!(r, [[9, 5, 1], [10, 6, 2], [11, 7, 3], [12, 8, 4]]);
    }

    #[test]
    fn test_rotate_pattern() {
        let (_, p) = parse_str("
            ***x*
            *.-.*
            *|*|*
            -.x.-
            *x***
        ");
        let p_rot = p.rot90();
        let (_, expected) = parse_str("
            *|***
            x.-.*
            *x*|*
            *.-.x
            *|***
        ");
        assert_eq!(p_rot, expected);
    }

    #[test]
    fn test_reflect_pattern() {
        let (_, p) = parse_str("
            ***x*
            *.-.x
            *|*|*
            -.x.-
            *x***
        ");
        let p_mirror = p.mirror();
        let (_, expected) = parse_str("
            *x***
            x.-.*
            *|*|*
            -.x.-
            ***x*
        ");
        assert_eq!(p_mirror, expected);
    }

    #[test]
    fn test_matches_cells() {
        let threes_ortho = PatternSolution::parse(
            "
                **3**
                *.*.*
                **3**
                *.*.*
                *****
            ",
            "
                *****
                *.-.*
                *****
                *.-.*
                *****
            ",
        );

        assert!(threes_ortho.try_match(
            &threes_ortho.cells,
            &threes_ortho.input.horizontals,
            &threes_ortho.input.verticals,
        ));

        let (cells2, _) = parse_str("
            2*3*1
            *.*.*
            **3*0
            *.*.*
            B*B*B
        ");
        assert!(threes_ortho.try_match(
            &cells2,
            &threes_ortho.input.horizontals,
            &threes_ortho.input.verticals,
        ));

        let (_, pat3) = parse_str("
            *x*x*
            -.-.-
            *x*x*
            -.-.-
            *x*x*
        ");
        assert!(threes_ortho.try_match(
            &threes_ortho.cells,
            &pat3.horizontals,
            &pat3.verticals,
        ));
    }

    #[test]
    fn test_matches_edge_forced_turn() {
        let threes_ortho = PatternSolution::parse(
            "
                *****
                x.-.*
                *x***
                *.*.*
                *****
            ",
            "
                *|***
                x.-.*
                *x***
                *.*.*
                *****
            ",
        );

        let rs = threes_ortho.rotations();

        let (cells, pat) = parse_str("
            *%*%*
            %.%.%
            *%*%*
            %.-.x
            *|*?*
        ");
        assert!(rs.iter().any(|p| p.try_match(&cells, &pat.horizontals, &pat.verticals)));
    }

    #[test]
    fn test_matches_three_in_corner() {
        let threes_ortho = PatternSolution::parse(
            "
                B*B**
                *.*.*
                B*3**
                *.*.*
                *****
            ",
            "
                *****
                *.-.*
                *|***
                *.*.*
                *****
            ",
        );

        let rs = threes_ortho.rotations();

        let (cells, pat) = parse_str("
            B%B%B
            %.%.%
            *%3%B
            %.-.x
            *|*?B
        ");
        assert!(rs.iter().any(|p| p.try_match(&cells, &pat.horizontals, &pat.verticals)));
    }
}
