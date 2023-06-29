use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    Any,
    OutOfBounds,
    Nothing,
    Zero,
    One,
    Two,
    Three,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Edge {
    Any,
    OutOfBounds,
    Unknown,
    Empty,
    Filled,
}

impl Cell {
    fn matches(&self, other: &Self) -> bool {
        return *self == Cell::Any || *other == Cell::Any || *self == *other;
    }
}

impl Edge {
    fn matches(&self, other: &Self) -> bool {
        return *self == Edge::Any
            || *other == Edge::Any
            || *self == *other
            || (*self == Edge::OutOfBounds && *other == Edge::Empty)
            || (*self == Edge::Empty && *other == Edge::OutOfBounds);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pattern {
    pub horizontals: Horizontals,
    pub verticals: Verticals,
}

pub type Horizontals = [[Edge; 3]; 2];
pub type Verticals = [[Edge; 2]; 3];
pub type CellWindow = [[Cell; 3]; 3];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PatternSolution {
    pub cells: CellWindow,
    pub input: Pattern,
    pub output: Pattern,
}

impl fmt::Display for PatternSolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();

        res.push_str("=====");
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
                    _ => " ",
                });

                res.push_str(match self.input.verticals[i][j] {
                    Edge::Filled => "|",
                    Edge::Empty => "x",
                    _ => " ",
                })
            }
            res.push_str(match self.cells[i].last().unwrap() {
                Cell::Zero => "0",
                Cell::One => "1",
                Cell::Two => "2",
                Cell::Three => "3",
                _ => " ",
            });

            res.push_str("*\n*");

            // horizontals
            if i < 2 {
                for j in 0..self.input.horizontals[i].len() {
                    res.push_str(match self.input.horizontals[i][j] {
                        Edge::Filled => ".-",
                        Edge::Empty => ".x",
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
                    _ => " ",
                });

                res.push_str(match self.output.verticals[i][j] {
                    Edge::Filled => "|",
                    Edge::Empty => "x",
                    _ => " ",
                })
            }
            res.push_str(match self.cells[i].last().unwrap() {
                Cell::Zero => "0",
                Cell::One => "1",
                Cell::Two => "2",
                Cell::Three => "3",
                _ => " ",
            });

            res.push_str("*\n");

            // horizontals
            if i < 2 {
                res.push_str("*");
                for j in 0..self.output.horizontals[i].len() {
                    res.push_str(match self.output.horizontals[i][j] {
                        Edge::Filled => ".-",
                        Edge::Empty => ".x",
                        _ => ". ",
                    })
                }
                res.push_str("*");
            }
            res.push_str("\n");
        }

        res.push_str("=====");

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
    let mut r = v.clone();
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
        let mut n = self.clone();
        n.verticals = rot90(&self.horizontals);
        n.horizontals = rot90(&self.verticals);
        n
    }

    fn mirror(&self) -> Pattern {
        let mut n = self.clone();
        n.verticals = mirror(&self.verticals);
        n.horizontals = mirror(&self.horizontals);
        n
    }
}

impl PatternSolution {
    fn rot90(&self) -> PatternSolution {
        let n = PatternSolution {
            cells: rot90(&self.cells),
            input: self.input.rot90(),
            output: self.output.rot90(),
        };
        n
    }

    fn reflect(&self) -> PatternSolution {
        let n = PatternSolution {
            cells: mirror(&self.cells),
            input: self.input.mirror(),
            output: self.output.mirror(),
        };
        n
    }
    pub fn rotations(&self) -> Vec<PatternSolution> {
        let mut res: Vec<PatternSolution> = vec![];
        res.push(self.clone());
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
        let r = h.iter().map(|&x| x).collect();
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
        for i in 0..horizontals.len() {
            for j in 0..horizontals[0].len() {
                if !self.input.horizontals[i][j].matches(&horizontals[i][j]) {
                    return false;
                }

                if !self.input.verticals[j][i].matches(&verticals[j][i]) {
                    return false;
                }
            }
        }

        for i in 0..cells.len() {
            for j in 0..cells.len() {
                if !self.cells[i][j].matches(&cells[i][j]) {
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
    fn test_rotate_array() {
        let r = rot90(&[[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
        assert!(r == [[9, 5, 1], [10, 6, 2], [11, 7, 3], [12, 8, 4]])
    }

    #[test]
    fn test_rotate_pattern() {
        let p = Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::Any],
                [Edge::Filled, Edge::Empty, Edge::Filled],
            ],
            verticals: [
                [Edge::Any, Edge::Empty],
                [Edge::Filled, Edge::Filled],
                [Edge::Empty, Edge::Any],
            ],
        };
        let p_rot = p.rot90();
        let expected = Pattern {
            horizontals: [
                [Edge::Empty, Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Filled, Edge::Empty],
            ],
            verticals: [
                [Edge::Filled, Edge::Any],
                [Edge::Empty, Edge::Filled],
                [Edge::Filled, Edge::Any],
            ],
        };

        assert!(p_rot == expected)
    }

    #[test]
    fn test_reflect_pattern() {
        let p = Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::Empty],
                [Edge::Filled, Edge::Empty, Edge::Filled],
            ],
            verticals: [
                [Edge::Any, Edge::Empty],
                [Edge::Filled, Edge::Filled],
                [Edge::Empty, Edge::Any],
            ],
        };
        let p_rot = p.mirror();

        let expected = Pattern {
            horizontals: [
                [Edge::Empty, Edge::Filled, Edge::Any],
                [Edge::Filled, Edge::Empty, Edge::Filled],
            ],
            verticals: [
                [Edge::Empty, Edge::Any],
                [Edge::Filled, Edge::Filled],
                [Edge::Any, Edge::Empty],
            ],
        };

        assert!(p_rot == expected)
    }

    #[test]
    fn test_matches_cells() {
        let threes_ortho = PatternSolution {
            output: Pattern {
                horizontals: [
                    [Edge::Any, Edge::Filled, Edge::Any],
                    [Edge::Any, Edge::Filled, Edge::Any],
                ],
                verticals: [
                    [Edge::Any, Edge::Any],
                    [Edge::Any, Edge::Any],
                    [Edge::Any, Edge::Any],
                ],
            },
            input: Pattern {
                horizontals: [[Edge::Any; 3]; 2],
                verticals: [[Edge::Any; 2]; 3],
            },
            cells: [
                [Cell::Any, Cell::Three, Cell::Any],
                [Cell::Any, Cell::Three, Cell::Any],
                [Cell::Any, Cell::Any, Cell::Any],
            ],
        };

        assert!(threes_ortho.try_match(
            &threes_ortho.cells,
            &threes_ortho.input.horizontals,
            &threes_ortho.input.verticals
        ));

        assert!(threes_ortho.try_match(
            &[
                [Cell::Two, Cell::Three, Cell::One],
                [Cell::Any, Cell::Three, Cell::Zero],
                [Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds],
            ],
            &threes_ortho.input.horizontals,
            &threes_ortho.input.verticals
        ));

        assert!(threes_ortho.try_match(
            &threes_ortho.cells,
            &[
                [Edge::Filled, Edge::Filled, Edge::Filled],
                [Edge::Filled, Edge::Filled, Edge::Filled],
            ],
            &[
                [Edge::Empty, Edge::Empty],
                [Edge::Empty, Edge::Empty],
                [Edge::Empty, Edge::Empty],
            ]
        ));
    }

    #[test]
    fn test_matches_edge_forced_turn() {
        let threes_ortho = PatternSolution {
            output: Pattern {
                horizontals: [
                    [Edge::Empty, Edge::Filled, Edge::Any],
                    [Edge::Any, Edge::Any, Edge::Any],
                ],
                verticals: [
                    [Edge::Filled, Edge::Any],
                    [Edge::Empty, Edge::Any],
                    [Edge::Any, Edge::Any],
                ],
            },
            input: Pattern {
                horizontals: [
                    [Edge::Empty, Edge::Filled, Edge::Any],
                    [Edge::Any, Edge::Any, Edge::Any],
                ],
                verticals: [
                    [Edge::Any, Edge::Any],
                    [Edge::Empty, Edge::Any],
                    [Edge::Any, Edge::Any],
                ],
            },
            cells: [
                [Cell::Any, Cell::Any, Cell::Any],
                [Cell::Any, Cell::Any, Cell::Any],
                [Cell::Any, Cell::Any, Cell::Any],
            ],
        };

        let rs = threes_ortho.rotations();

        for r in rs.clone() {
            println!("{r}")
        }

        

        // assert!(rs.iter().any(|p| p.try_match(
        //     &threes_ortho.cells,
        //     &[
        //         [Edge::OutOfBounds, Edge::Filled, Edge::Empty],
        //         [Edge::OutOfBounds, Edge::Unknown, Edge::Filled],
        //     ],
        //     &[
        //         [Edge::OutOfBounds, Edge::OutOfBounds],
        //         [Edge::Filled, Edge::Unknown],
        //         [Edge::Unknown, Edge::Empty],
        //     ]
        // )));
        assert!(rs.iter().any(|p| p.try_match(
            &threes_ortho.cells,
            &[
                [Edge::OutOfBounds, Edge::OutOfBounds, Edge::OutOfBounds],
                [Edge::OutOfBounds, Edge::Filled, Edge::Empty],
                
            ],
            &[
                [Edge::OutOfBounds, Edge::OutOfBounds],
                [Edge::OutOfBounds, Edge::OutOfBounds],
                [Edge::Filled, Edge::Unknown],
            ]
        )));
    }
}
