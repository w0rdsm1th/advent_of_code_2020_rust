
pub fn part1(puzzle_input: Vec<i32>){
    /*
     - combinations with double for-loop for each combination of 2 numbers
     - recursive call to a helper function
        pros: easily extensible beyond 2020 and 2 digits
     */
    for (outer_idx, number) in puzzle_input.iter().enumerate() {
        for (inner_idx, inner_number) in puzzle_input.iter().enumerate() {
            if outer_idx == inner_idx {continue}  // not allowing using same number twice
            if 2020 - number - inner_number == 0 {
                println!("found! {} * {} = {}", number, inner_number, number * inner_number);
            }
        }
    }
}

pub fn part2(puzzle_input: Vec<i32>){
    /*
     - combinations with double for-loop for each combination of 2 numbers
     - recursive call to a helper function
        pros: easily extensible beyond 2020 and 2 digits
     */
    for (outer_idx, number) in puzzle_input.iter().enumerate() {
        for (middle_idx, middle_number) in puzzle_input.iter().enumerate() {
            if outer_idx == middle_idx {continue}  // not allowing using same number twice
            for (inner_idx, inner_number) in puzzle_input.iter().enumerate() {
                if outer_idx == inner_idx {continue}  // not allowing using same number twice
                if 2020 - number - middle_number - inner_number == 0 {
                    println!("found! {} * {} * {} = {}", number, middle_number, inner_number, number * middle_number * inner_number);
                }
            }
        }
    }
}
pub fn part2_recursive(puzzle_input: Vec<i32>){
    /*
    INCOMPLETE
     - recursive call to a helper function
        pros: easily extensible beyond 2020 and 2 digits

     */
    for (outer_idx, number) in puzzle_input.iter().enumerate() {
        
    }
}

#[cfg(test)]
mod tests {
    use crate::day1::{part1, part2};

    #[test]
    fn test_part1() {
        let inp = vec![1721, 979, 366, 299, 675, 1456];
        // assert_eq!(solution(inp), 514579)
        part1(inp)
    }
    #[test]
    fn test_part2() {
        let inp = vec![1721, 979, 366, 299, 675, 1456];
        // assert_eq!(solution(inp), 514579)
        part2(inp)
    }
}

/* LEARNINGS:
- loop, while, while let, for
All four types of loop support break expressions, continue expressions, and labels. Only loop supports evaluation to non-trivial values.
*/