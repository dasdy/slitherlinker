use crate::data::pattern::Cell;
use crate::data::pattern::Edge;
use crate::data::pattern::Pattern;
use crate::data::pattern::PatternSolution;
use std::collections::BTreeMap;

fn push(r: &mut BTreeMap<String, PatternSolution>, prefix: &str, pattern: &PatternSolution) {
    pattern.rotations().iter().enumerate().for_each(|(i, &p)| {
        r.insert(format!("{prefix}-{i}"), p);
    });
}

pub fn patterns() -> BTreeMap<String, PatternSolution> {
    let mut r = BTreeMap::new();

    // Each pattern is two strings (input, output), each with 5 non-empty lines:
    //   cell-row:  c v c v c   (5 chars: cells at 0,2,4; vertical edges at 1,3)
    //   horiz-row: h . h . h   (5 chars: horizontal edges at 0,2,4; filler '.' at 1,3)
    //   cell-row
    //   horiz-row
    //   cell-row
    //
    // Cell chars:          *=Any  0-3=value  B=OutOfBounds  .=Nothing
    // Vertical edge chars: *=Any  |=Filled   x=Empty  X=EmptyStrict  %=OutOfBounds  ?=Unknown
    // Horiz edge chars:    *=Any  -=Filled   x=Empty  X=EmptyStrict  %=OutOfBounds  ?=Unknown
    // Leading/trailing whitespace on each line is ignored, so indentation is free.
    // Output cell chars are ignored; cells are always taken from the input string.

    push(
        &mut r,
        "two threes diagonally",
        &PatternSolution::parse(
            "
            3****
            *.*.*
            **3**
            *.*.*
            *****
        ",
            "
            *****
            *.*.*
            ***|*
            *.-.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "two with line incoming",
        &PatternSolution::parse(
            "
            *****
            *.x.*
            **2**
            -.*.*
            *****
        ",
            "
            *****
            *.x.*
            ***|*
            -.*.*
            *x***
        ",
        ),
    );

    push(
        &mut r,
        "three with line incoming",
        &PatternSolution::parse(
            "
            *****
            *.*.*
            **3**
            -.*.*
            *****
        ",
            "
            *****
            *.-.*
            ***|*
            -.*.*
            *x***
        ",
        ),
    );

    push(
        &mut r,
        "two ones diagonally - inner",
        &PatternSolution::parse(
            "
            1x***
            x.*.*
            **1**
            *.*.*
            *****
        ",
            "
            *x***
            x.x.*
            *x***
            *.*.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "corner",
        &PatternSolution::parse(
            "
            *****
            *.-.*
            *|***
            *.*.*
            *****
        ",
            "
            *x***
            x.-.*
            *|***
            *.*.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "one in corner",
        &PatternSolution::parse(
            "
            B*B*B
            *.*.*
            B*1**
            *.*.*
            B****
        ",
            "
            *****
            *.x.*
            *x***
            *.*.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "two in corner",
        &PatternSolution::parse(
            "
            B*B*B
            *.*.*
            B*2**
            *.*.*
            B****
        ",
            "
            *****
            *.*.-
            *****
            *.*.*
            *|***
        ",
        ),
    );

    push(
        &mut r,
        "three in corner",
        &PatternSolution::parse(
            "
            B*B*B
            *.*.*
            B*3**
            *.*.*
            B****
        ",
            "
            *****
            *.-.*
            *|***
            *.*.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "three orthoganally",
        &PatternSolution::parse(
            "
            **3**
            *.*.*
            **3**
            *.*.*
            *****
        ",
            "
            *****
            x.-.x
            *****
            *.-.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "three with a missing edge",
        &PatternSolution::parse(
            "
            *X***
            *.X.*
            *X***
            *.*.*
            *****
        ",
            "
            *x***
            x.x.*
            *x***
            *.*.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "filled one",
        &PatternSolution::parse(
            "
            *****
            *.-.*
            **1**
            *.*.*
            *****
        ",
            "
            *****
            *.-.*
            *x*x*
            *.x.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "one with three emptн edges",
        &PatternSolution::parse(
            "
            *****
            *.*.*
            *x1x*
            *.x.*
            *****
        ",
            "
            *****
            *.-.*
            *x*x*
            *.x.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "two with two empty edges connected",
        &PatternSolution::parse(
            "
            *****
            *.*.*
            **2x*
            *.x.*
            *****
        ",
            "
            *****
            *.-.*
            *|*x*
            *.x.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "two with two empty edges opposite",
        &PatternSolution::parse(
            "
            *****
            *.x.*
            **2**
            *.x.*
            *****
        ",
            "
            *****
            *.x.*
            *|*|*
            *.x.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "two with two filled edges connected",
        &PatternSolution::parse(
            "
            *****
            *.-.*
            *|2**
            *.*.*
            *****
        ",
            "
            *****
            *.-.*
            *|*x*
            *.x.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "two with two filled edges opposite",
        &PatternSolution::parse(
            "
            *****
            *.*.*
            *|2|*
            *.*.*
            *****
        ",
            "
            *****
            *.x.*
            *|*|*
            *.x.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "three with one empty",
        &PatternSolution::parse(
            "
            *****
            *.*.*
            **3**
            *.X.*
            *****
        ",
            "
            *****
            *.-.*
            *|*|*
            *.x.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "three with three edges",
        &PatternSolution::parse(
            "
            *****
            *.-.*
            *|3|*
            *.*.*
            *****
        ",
            "
            *****
            *.-.*
            *|*|*
            *.x.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "forced edge continue",
        &PatternSolution::parse(
            "
            *x***
            ?.-.*
            *x***
            *.*.*
            *****
        ",
            "
            *x***
            -.-.*
            *x***
            *.*.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "forced edge turn",
        &PatternSolution::parse(
            "
            *****
            -.x.*
            *x***
            *.*.*
            *****
        ",
            "
            *|***
            -.x.*
            *x***
            *.*.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "forced edge turn (RHS)",
        &PatternSolution::parse(
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
        ),
    );

    push(
        &mut r,
        "forced edge turn (OOB)",
        &PatternSolution::parse(
            "
            *****
            *.-.%
            ***x*
            *.*.*
            *****
        ",
            "
            ***|*
            *.-.%
            ***x*
            *.*.*
            *****
        ",
        ),
    );

    push(
        &mut r,
        "forced edge turn (OOB)(RHS)",
        &PatternSolution::parse(
            "
            *****
            %.-.*
            *x***
            *.*.*
            *****
        ",
            "
            *|***
            %.-.*
            *x***
            *.*.*
            *****
        ",
        ),
    );

    r.insert(
        String::from("zero"),
        PatternSolution::parse(
            "
            *****
            *.*.*
            **0**
            *.*.*
            *****
        ",
            "
            *****
            *.x.*
            *x*x*
            *.x.*
            *****
        ",
        ),
    );

    r
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_patterns_from_str_matches_patterns() {
        let expected = patterns();
        let actual = patterns();

        // Report mismatches clearly before asserting equality.
        for (k, v) in &expected {
            match actual.get(k) {
                None => panic!("patterns_from_str() is missing key {k:?}"),
                Some(a) if a != v => {
                    panic!("Mismatch for key {k:?}:\nexpected:\n{v}\nactual:\n{a}")
                }
                _ => {}
            }
        }
        for k in actual.keys() {
            if !expected.contains_key(k) {
                panic!("patterns_from_str() has unexpected key {k:?}");
            }
        }
    }
}
