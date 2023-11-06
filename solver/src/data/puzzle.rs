use crate::data::pattern::Edge;
use crate::parse::Cell;

#[derive(Debug, Clone)]
pub struct EdgeCreateError;

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub cells: Vec<Vec<Cell>>,
    pub xsize: usize,
    pub ysize: usize,
}

impl Puzzle {
    pub fn from<const W: usize, const H: usize>(cells: &[[Cell; W]; H]) -> Puzzle {
        Puzzle {
            xsize: W,
            ysize: H,
            cells: cells.iter().map(|row| row.to_vec()).collect(),
        }
    }

    pub fn edges<const W1: usize, const H1: usize, const W2: usize, const H2: usize>(
        horizontals: &[[Edge; W1]; H1],
        verticals: &[[Edge; W2]; H2],
    ) -> Result<Vec<Edge>, EdgeCreateError> {
        if W1 + 1 != W2 || H1 != H2 + 1 || W1 < 1 || H1 < 1 || W2 < 1 || H2 < 1 {
            Err(EdgeCreateError {})
        } else {
            let mut result = vec![];

            horizontals.iter().for_each(|row| result.extend_from_slice(row));
            verticals.iter().for_each(|row| result.extend_from_slice(row));

            Ok(result)
        }
    }
    pub fn edge_ix(&self, i: usize, j: usize, is_horizontal: bool) -> usize {
        if is_horizontal {
            i * self.xsize + j
        } else {
            ((1 + self.xsize) * self.ysize) + (i * (1 + self.ysize) + j)
        }
    }

    pub fn edges_around_cell(&self, i: usize, j: usize) -> (usize, usize, usize, usize) {
        (
            self.edge_ix(i, j, true),
            self.edge_ix(i + 1, j, true),
            self.edge_ix(i, j, false),
            self.edge_ix(i, j + 1, false),
        )
    }

    pub fn edges_around_edge(&self, ix: usize) -> Vec<usize> {
        let mut res = vec![];

        let is_horizontal = ix < (1 + self.xsize) * self.ysize;
        // println!("ix: {ix}, xsize: {}, ysize: {}", self.xsize, self.ysize);
        let simpler_ix = if is_horizontal {
            ix
        } else {
            ix - ((1 + self.xsize) * self.ysize)
        };
        let (i, j) = if is_horizontal {
            (simpler_ix / self.xsize, simpler_ix % self.ysize)
        } else {
            (simpler_ix / (self.ysize + 1), simpler_ix % (self.ysize + 1))
        };

        // println!("i: {i}, j: {j}, horizontal?: {is_horizontal}");

        if is_horizontal {
            // left side
            if j > 0 {
                // println!("L-Hor");
                res.push(self.edge_ix(i, j - 1, true))
            }
            if i > 0 {
                // println!("L-Up");
                res.push(self.edge_ix(i - 1, j, false))
            }
            if i < self.xsize {
                // println!("L-Down");
                res.push(self.edge_ix(i, j, false))
            }

            // right side
            if j + 1 < self.ysize {
                // println!("R-Hor");
                res.push(self.edge_ix(i, j + 1, true));
            }
            if j < self.ysize && i < self.xsize {
                // println!("R-Down");
                res.push(self.edge_ix(i, j + 1, false));
            }
            if j < self.ysize && i > 0 {
                // println!("R-Up");
                res.push(self.edge_ix(i - 1, j + 1, false));
            }
        } else {
            // up side
            if i > 0 {
                // println!("U-Ver");
                res.push(self.edge_ix(i - 1, j, false))
            }
            if j < self.ysize {
                // println!("U-right");
                res.push(self.edge_ix(i, j, true))
            }
            if j > 0 {
                // println!("U-left");
                res.push(self.edge_ix(i, j - 1, true))
            }

            // down side

            if i + 1 < self.xsize {
                // println!("D-Ver");
                res.push(self.edge_ix(i + 1, j, false))
            }
            if i < self.xsize && j < self.ysize {
                // println!("D-right");
                res.push(self.edge_ix(i + 1, j, true))
            }
            if j > 0 && i < self.xsize {
                // println!("D-left");
                res.push(self.edge_ix(i + 1, j - 1, true))
            }
        }

        // println!("Edges around {ix}: {:?}", res);
        res.sort();

        res
    }

    pub fn edges_around_point(&self, i: usize, j: usize) -> Vec<usize> {
        let mut res = vec![];
        if i > 0 {
            res.push(self.edge_ix(i - 1, j, false))
        }
        if j > 0 {
            res.push(self.edge_ix(i, j - 1, true))
        }
        if i < self.xsize {
            res.push(self.edge_ix(i, j, false))
        }
        if j < self.ysize {
            res.push(self.edge_ix(i, j, true))
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Puzzle;

    #[test]
    fn test_indices_2() {
        let p = Puzzle::from(&[[-1; 2]; 2]);
        let e = p.edges_around_cell(1, 1);
        assert_eq!(e, (3, 5, 10, 11));
    }

    #[test]
    fn test_indices_10() {
        let p = Puzzle::from(&[[-1; 10]; 10]);
        let e = p.edges_around_cell(9, 9);
        assert_eq!(e, (99, 109, 218, 219));

        let e = p.edges_around_cell(0, 0);
        assert_eq!(e, (0, 10, 110, 111));
    }

    #[test]
    fn test_lines_around_0() {
        let p = Puzzle::from(&[[-1; 2]; 2]);
        let e = p.edges_around_edge(0);
        assert_eq!(e, [1, 6, 7]);

        let e = p.edges_around_edge(1);
        assert_eq!(e, [0, 7, 8]);

        let e = p.edges_around_edge(2);
        assert_eq!(e, [3, 6, 7, 9, 10]);

        let e = p.edges_around_edge(3);
        assert_eq!(e, [2, 7, 8, 10, 11]);

        let e = p.edges_around_edge(4);
        assert_eq!(e, [5, 9, 10]);

        let e = p.edges_around_edge(5);
        assert_eq!(e, [4, 10, 11]);

        let e = p.edges_around_edge(6);
        assert_eq!(e, [0, 2, 9]);

        let e = p.edges_around_edge(7);
        assert_eq!(e, [0, 1, 2, 3, 10]);

        let e = p.edges_around_edge(8);
        assert_eq!(e, [1, 3, 11]);

        let e = p.edges_around_edge(9);
        assert_eq!(e, [2, 4, 6]);

        let e = p.edges_around_edge(10);
        assert_eq!(e, [2, 3, 4, 5, 7]);

        let e = p.edges_around_edge(11);
        assert_eq!(e, [3, 5, 8]);
    }

    #[test]
    fn test_edges_around_point() {
        let p = Puzzle::from(&[[-1; 2]; 2]);

        let e = p.edges_around_point(0, 0);
        assert_eq!(e, [6, 0]);
        let e = p.edges_around_point(0, 1);
        assert_eq!(e, [0, 7, 1]);
        let e = p.edges_around_point(0, 2);
        assert_eq!(e, [1, 8]);
        let e = p.edges_around_point(1, 0);
        assert_eq!(e, [6, 9, 2]);
        let e = p.edges_around_point(1, 1);
        assert_eq!(e, [7, 2, 10, 3]);
        let e = p.edges_around_point(1, 2);
        assert_eq!(e, [8, 3, 11]);
        let e = p.edges_around_point(2, 0);
        assert_eq!(e, [9, 4]);
        let e = p.edges_around_point(2, 1);
        assert_eq!(e, [10, 4, 5]);
        let e = p.edges_around_point(2, 2);
        assert_eq!(e, [11, 5]);
    }
}
