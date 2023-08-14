use crate::data::pattern::Cell;
use crate::data::pattern::Edge;
use crate::data::pattern::Pattern;
use crate::data::pattern::PatternSolution;
use std::collections::HashMap;

pub fn patterns() -> HashMap<String, PatternSolution> {
    let any_input = Pattern {
        horizontals: [[Edge::Any; 3]; 2],
        verticals: [[Edge::Any; 2]; 3],
    };

    let threes_corner = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Any, Edge::Any, Edge::Any],
                [Edge::Any, Edge::Filled, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Any, Edge::Filled],
                [Edge::Any, Edge::Any],
            ],
        },
        input: any_input,
        cells: [
            [Cell::Three, Cell::Any, Cell::Any],
            [Cell::Any, Cell::Three, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
        ],
    };

    let ones_diagonally_inner = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Empty, Edge::Empty, Edge::Any],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Empty, Edge::Any],
                [Edge::Empty, Edge::Any],
                [Edge::Any, Edge::Any],
            ],
        },
        input: Pattern {
            horizontals: [
                [Edge::Empty, Edge::Any, Edge::Any],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Empty, Edge::Any],
                [Edge::Any, Edge::Any],
                [Edge::Any, Edge::Any],
            ],
        },
        cells: [
            [Cell::One, Cell::Any, Cell::Any],
            [Cell::Any, Cell::One, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
        ],
    };

    let two_with_a_line = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Any, Edge::Empty, Edge::Any],
                [Edge::Filled, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Any, Edge::Filled],
                [Edge::Empty, Edge::Any],
            ],
        },
        input: Pattern {
            horizontals: [
                [Edge::Any, Edge::Empty, Edge::Any],
                [Edge::Filled, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Any, Edge::Any],
                [Edge::Any, Edge::Any],
            ],
        },
        cells: [
            [Cell::Any, Cell::Any, Cell::Any],
            [Cell::Any, Cell::Two, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
        ],
    };

    let three_with_a_line = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::Any],
                [Edge::Filled, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Any, Edge::Filled],
                [Edge::Empty, Edge::Any],
            ],
        },
        input: Pattern {
            horizontals: [
                [Edge::Any, Edge::Any, Edge::Any],
                [Edge::Filled, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Any, Edge::Any],
                [Edge::Any, Edge::Any],
            ],
        },
        cells: [
            [Cell::Any, Cell::Any, Cell::Any],
            [Cell::Any, Cell::Three, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
        ],
    };


    let threes_ortho = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Empty, Edge::Filled, Edge::Empty],
                [Edge::Any, Edge::Filled, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Any, Edge::Any],
                [Edge::Any, Edge::Any],
            ],
        },
        input: any_input.clone(),
        cells: [
            [Cell::Any, Cell::Three, Cell::Any],
            [Cell::Any, Cell::Three, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
        ],
    };

    let zero = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Any, Edge::Empty, Edge::Any],
                [Edge::Any, Edge::Empty, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Empty, Edge::Empty],
                [Edge::Any, Edge::Any],
            ],
        },
        input: any_input.clone(),
        cells: [
            [Cell::Any, Cell::Any, Cell::Any],
            [Cell::Any, Cell::Zero, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
        ],
    };

    let three_with_empty = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Empty, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Filled, Edge::Filled],
                [Edge::Any, Edge::Any],
            ],
        },
        input: Pattern {
            horizontals: [
                [Edge::Any, Edge::Any, Edge::Any],
                [Edge::Any, Edge::EmptyStrict, Edge::Any],
            ],
            verticals: [[Edge::Any; 2]; 3],
        },
        cells: [
            [Cell::Any, Cell::Any, Cell::Any],
            [Cell::Any, Cell::Three, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
        ],
    };

    let three_with_three_edges = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Empty, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Filled, Edge::Filled],
                [Edge::Any, Edge::Any],
            ],
        },
        input: Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Filled, Edge::Filled],
                [Edge::Any, Edge::Any],
            ],
        },
        cells: [
            [Cell::Any, Cell::Any, Cell::Any],
            [Cell::Any, Cell::Three, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
        ],
    };

    let two_with_two_empty_corner = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Empty, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Filled, Edge::Empty],
                [Edge::Any, Edge::Any],
            ],
        },
        input: Pattern {
            horizontals: [
                [Edge::Any, Edge::Any, Edge::Any],
                [Edge::Any, Edge::Empty, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Any, Edge::Empty],
                [Edge::Any, Edge::Any],
            ],
        },
        cells: [
            [Cell::Any, Cell::Any, Cell::Any],
            [Cell::Any, Cell::Two, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
        ],
    };

    let two_with_two_empty_ortho = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Any, Edge::Empty, Edge::Any],
                [Edge::Any, Edge::Empty, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Filled, Edge::Filled],
                [Edge::Any, Edge::Any],
            ],
        },
        input: Pattern {
            horizontals: [
                [Edge::Any, Edge::Empty, Edge::Any],
                [Edge::Any, Edge::Empty, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Any, Edge::Any],
                [Edge::Any, Edge::Any],
            ],
        },
        cells: [
            [Cell::Any, Cell::Any, Cell::Any],
            [Cell::Any, Cell::Two, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
        ],
    };

    let two_with_two_filled_corner = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Empty, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Filled, Edge::Empty],
                [Edge::Any, Edge::Any],
            ],
        },
        input: Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Any],
            ],
        },
        cells: [
            [Cell::Any, Cell::Any, Cell::Any],
            [Cell::Any, Cell::Two, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
        ],
    };

    let two_with_two_filled_ortho = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Any, Edge::Empty, Edge::Any],
                [Edge::Any, Edge::Empty, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Filled, Edge::Filled],
                [Edge::Any, Edge::Any],
            ],
        },
        input: Pattern {
            horizontals: [
                [Edge::Any, Edge::Any, Edge::Any],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Filled, Edge::Filled],
                [Edge::Any, Edge::Any],
            ],
        },
        cells: [
            [Cell::Any, Cell::Any, Cell::Any],
            [Cell::Any, Cell::Two, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
        ],
    };

    let one_with_one_filled = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Empty, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Empty, Edge::Empty],
                [Edge::Any, Edge::Any],
            ],
        },
        input: Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [[Edge::Any; 2]; 3],
        },
        cells: [
            [Cell::Any, Cell::Any, Cell::Any],
            [Cell::Any, Cell::One, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
        ],
    };

    let one_with_three_empty = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Empty, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Empty, Edge::Empty],
                [Edge::Any, Edge::Any],
            ],
        },
        input: Pattern {
            horizontals: [
                [Edge::Any, Edge::Any, Edge::Any],
                [Edge::Any, Edge::Empty, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Empty, Edge::Empty],
                [Edge::Any, Edge::Any],
            ],
        },
        cells: [
            [Cell::Any, Cell::Any, Cell::Any],
            [Cell::Any, Cell::One, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
        ],
    };

    let three_in_corner = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Any],
            ],
        },
        input: any_input.clone(),
        cells: [
            [Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds],
            [Cell::OutOfBounds, Cell::Three, Cell::Any],
            [Cell::OutOfBounds, Cell::Any, Cell::Any],
        ],
    };

    let two_in_corner = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Any, Edge::Any, Edge::Filled],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Any, Edge::Any],
                [Edge::Filled, Edge::Any],
            ],
        },
        input: any_input.clone(),
        cells: [
            [Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds],
            [Cell::OutOfBounds, Cell::Two, Cell::Any],
            [Cell::OutOfBounds, Cell::Any, Cell::Any],
        ],
    };

    let one_in_corner = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Any, Edge::Empty, Edge::Any],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Empty, Edge::Any],
                [Edge::Any, Edge::Any],
            ],
        },
        input: any_input.clone(),
        cells: [
            [Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds],
            [Cell::OutOfBounds, Cell::One, Cell::Any],
            [Cell::OutOfBounds, Cell::Any, Cell::Any],
        ],
    };

    let three_missing_edges = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Empty, Edge::Empty, Edge::Any],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Empty, Edge::Any],
                [Edge::Empty, Edge::Any],
                [Edge::Any, Edge::Any],
            ],
        },
        input: Pattern {
            horizontals: [
                [Edge::Any, Edge::Empty, Edge::Any],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Empty, Edge::Any],
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

    let forced_edge_continue = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Filled, Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Empty, Edge::Any],
                [Edge::Empty, Edge::Any],
                [Edge::Any, Edge::Any],
            ],
        },
        input: Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Empty, Edge::Any],
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

    let forced_edge_turn = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Filled, Edge::Empty, Edge::Any],
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
                [Edge::Filled, Edge::Empty, Edge::Any],
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

    let forced_edge_turn_oob = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::OutOfBounds],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Filled],
                [Edge::Any, Edge::Empty],
                [Edge::Any, Edge::Any],
            ],
        },
        input: Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::OutOfBounds],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Any, Edge::Empty],
                [Edge::Any, Edge::Any],
            ],
        },
        cells: [
            [Cell::Any, Cell::Any, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
        ],
    };

    let forced_edge_turn_oob_right = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::OutOfBounds, Edge::Filled, Edge::Any],
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
                [Edge::OutOfBounds, Edge::Filled, Edge::Any],
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

    let forced_edge_turn_right = PatternSolution {
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

    let edges_corner = PatternSolution {
        output: Pattern {
            horizontals: [
                [Edge::Empty, Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Empty, Edge::Any],
                [Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Any],
            ],
        },
        input: Pattern {
            horizontals: [
                [Edge::Any, Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Any, Edge::Any],
            ],
            verticals: [
                [Edge::Any, Edge::Any],
                [Edge::Filled, Edge::Any],
                [Edge::Any, Edge::Any],
            ],
        },
        cells: [
            [Cell::Any, Cell::Any, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
            [Cell::Any, Cell::Any, Cell::Any],
        ],
    };

    let mut r: HashMap<String, PatternSolution> = HashMap::new();

    push(&mut r, "two threes diagonally", &threes_corner);
    push(&mut r, "two with line incoming", &two_with_a_line);
    push(&mut r, "three with line incoming", &three_with_a_line);
    push(&mut r, "two ones diagonally - inner", &ones_diagonally_inner);
    push(&mut r, "corner", &edges_corner);
    push(&mut r, "one in corner", &one_in_corner);
    push(&mut r, "two in corner", &two_in_corner);
    push(&mut r, "three in corner", &three_in_corner);
    push(&mut r, "three orthoganally", &threes_ortho);
    push(&mut r, "three with a missing edge", &three_missing_edges);
    push(&mut r, "filled one", &one_with_one_filled);
    push(&mut r, "one with three emptн edges", &one_with_three_empty);
    push(&mut r, "two with two empty edges connected",&two_with_two_empty_corner);
    push(&mut r, "two with two empty edges opposite",&two_with_two_empty_ortho);
    push(&mut r, "two with two filled edges connected",&two_with_two_filled_corner);
    push(&mut r, "two with two filled edges opposite",&two_with_two_filled_ortho);
    push(&mut r,"three with one empty",&three_with_empty);
    push(&mut r,"three with three edges",&three_with_three_edges);
    push(&mut r,"forced edge continue",&forced_edge_continue);
    push(&mut r, "forced edge turn", &forced_edge_turn);
    push(&mut r,"forced edge turn (RHS)",&forced_edge_turn_right);
    push(&mut r,"forced edge turn (OOB)",&forced_edge_turn_oob);
    push(&mut r,"forced edge turn (OOB)(RHS)",&forced_edge_turn_oob_right);
    r.insert(String::from("zero"), zero);
    r
}

fn push(r: &mut HashMap<String, PatternSolution>, prefix: &str, pattern: &PatternSolution) {
    pattern.rotations().iter().enumerate().for_each(|(i, &p)| {
        r.insert(format!("{prefix}-{i}"), p);
    });
}
