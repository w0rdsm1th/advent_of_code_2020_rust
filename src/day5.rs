/*
project: advent_of_code_2020_rust
created by: aleks
on: 21 September 2021
*/

fn parse_boarding_pass(inp: &str) -> ((i32, i32), (i32, i32)) {
    // binary space partitioning
    let mut row_space = (0, 127);
    let mut column_space = (0, 7);

    fn helper(lower: &i32, upper: &i32) -> i32 {
        let combined = lower + upper;
        combined / 2
    }

    // pop off the first char of input
    for instruction in inp.chars() {
        // rows - lower half
        if instruction == 'F' {
            row_space = (row_space.0, helper(&row_space.0, &row_space.1));
        }
        // rows - upper half
        else if instruction == 'B' {
            row_space = (helper(&row_space.0, &row_space.1) + 1, row_space.1);
        }
        // columns - lower half
        else if instruction == 'L' {
            column_space = (column_space.0, helper(&column_space.0, &column_space.1));
        }
        // columns - upper half
        else if instruction == 'R' {
            column_space = (helper(&column_space.0, &column_space.1) + 1, column_space.1);
        } else { panic!("unexpected instruction: {:?}", instruction) }
        // dbg!(&instruction, &row_space);
    }
    (row_space, column_space)
}

fn seat_unique_id(inp: &((i32, i32), (i32, i32))) -> i32 {
    assert_eq!(inp.0.0, inp.0.1);
    assert_eq!(inp.1.0, inp.1.1);
    return inp.0.0 * 8 + inp.1.0;

}

pub fn part1(inp: Vec<&str>) -> i32 {
    /* */
    let mut current_max = 0;
    for boarding_pass in inp {
        let seat_number = parse_boarding_pass(boarding_pass);
        let this_id = seat_unique_id(&seat_number);
        if this_id > current_max {
            current_max = this_id;
        }
    }
    dbg!(current_max);
    current_max
}

pub fn part2() {
    /* */
}


#[cfg(test)]
mod tests {
    use super::{part1, part2, parse_boarding_pass, seat_unique_id};


    #[test]
    fn test_seat_unique_id() {
        for (seat_in, expected) in vec![
            (((44, 44), (5, 5)), 357),
            (((70, 70), (7, 7)), 567),
            (((14, 14), (7, 7)), 119),
            (((102, 102), (4, 4)), 820),
        ] {
            assert_eq!(seat_unique_id(&seat_in), expected)
        }
    }

    #[test]
    fn test_row_int_division() {
        // row partitions
        dbg!(127 / 2);  // F ✔
        dbg!((63) / 2);  // B ❌ 31 != 32
        dbg!((63 + 32) / 2);  // F ✔
        dbg!((32 + 47) / 2);  // B ❌ 39 != 40
        dbg!((40 + 47) / 2);  // B ❌ 43 != 44
    }

    #[test]
    fn test_col_int_division() {
        // column partitions
        dbg!((7+0)/2, 4);  // R ❌, upper half, 3 != 4
        dbg!((7 + 4) / 2);  // L ✔, lower half, == 5
        dbg!((4 + 5) / 2);  // R, upper half, == 5
    }

    #[test]
    fn test_columns() {
        let boarding_pass = "RLR";
        let output = parse_boarding_pass(boarding_pass);
        // dbg!(output);
        assert_eq!(output, ((0, 127), (5, 5)))
    }

    #[test]
    fn test_parse_boarding_pass_small() {
        // BFFFBBFRRR: row 70, column 7, seat ID 567.
        // FFFBBBFRRR: row 14, column 7, seat ID 119.
        // BBFFBBFRLL: row 102, column 4, seat ID 820.
        let mut boarding_pass = "F".to_string();
        assert_eq!(parse_boarding_pass(&boarding_pass), ((0, 63), (0, 7)));
        boarding_pass = "FB".to_string();
        assert_eq!(parse_boarding_pass(&boarding_pass), ((32, 63), (0, 7)));
        boarding_pass = "FBF".to_string();
        assert_eq!(parse_boarding_pass(&boarding_pass), ((32, 47), (0, 7)));
    }

    #[test]
    fn test_parse_boarding_pass() {
        for (input, output) in vec![
            ("FBFBBFFRLR", ((44, 44), (5, 5))),
            ("BFFFBBFRRR", ((70, 70), (7, 7))),
            ("FFFBBBFRRR", ((14, 14), (7, 7))),
            ("BBFFBBFRLL", ((102, 102), (4, 4))),
        ] {
            assert_eq!(parse_boarding_pass(&input), output)
        }
    }
}
/* LEARNINGS: 
-
*/