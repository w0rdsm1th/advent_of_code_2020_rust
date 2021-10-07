/*
project: advent_of_code_2020_rust
created by: aleks
on: 07 April 2021
*/

// magic pattern matching -
// https://stackoverflow.com/a/39879622/3596968
fn part1_password_checker((lb, ub, letter, candidate_pw): (i8, i8, char, &str)) -> bool {
    // 1-5 m: zmmmsm
    let c = candidate_pw.to_string().matches(letter).count() as i8;
    lb <= c && c <= ub
}

fn part2_password_checker((pos1, pos2, letter, candidate_pw): (i8, i8, char, &str)) -> bool {
    // no concept of 0-index so decrement nth pos by 1
    let first = candidate_pw.chars().nth((pos1 - 1) as usize);
    let second = candidate_pw.chars().nth((pos2 - 1) as usize);

    // one of, but not both of the candidate letters
    ((first != second) & ((first == Some(letter)) | (second == Some(letter))))
}

fn parse_line(password_line: &str)  -> (i8, i8, char, &str) {
    // format of each line, number of times must appear, the letter, and the password
    // 1-5 m: zmmmsm
    let split_line: Vec<&str> = password_line.split_whitespace().collect();
    let bounds: Vec<i8> = (split_line[0]).split('-').map(|c| c.trim().parse::<i8>().unwrap()).collect();
    // println!("bounds are: {:?}", bounds);

    (bounds[0], bounds[1], split_line[1].chars().next().unwrap(), split_line[2])
}

pub fn part1(puzzle_input: Vec<&str>) -> i32 {
    /* how many passwords are valid, according to their policies*/

    let mut counter = 0;
    for i in puzzle_input {
        if part1_password_checker(parse_line(i)) {
            counter += 1;
        }
        // puzzle_input.iter().filter(|&n| password_checker(parse_line(n))).count();
    }
    println!("Part 1: {:?}", counter);
    counter
}

pub fn part2(puzzle_input: Vec<&str>) -> i32 {
    /*
    no concept of 0-index
    */
    let mut counter = 0;
    for i in puzzle_input {
        if part2_password_checker(parse_line(i)) {
            counter += 1;
        }
    }
    println!("Part 2: {:?}", counter);
    counter
}

#[cfg(test)]
mod tests {
    use super::{part1_password_checker, part2_password_checker, parse_line, part1, part2};

    #[test]
    fn test_part1_password_checker() {
        assert_eq!(part1_password_checker((1, 5, 'm', "zmmmsm")), true);
        assert_eq!(part1_password_checker((1, 5, 'm', "zaaas")), false);
        assert_eq!(part1_password_checker((1, 5, 'q', "zaaas")), false);
    }
    #[test]
    fn test_part2_password_checker() {
        assert_eq!(part2_password_checker((1, 3, 'a', "abcde")), true);
        assert_eq!(part2_password_checker((1, 3, 'b', "cdefg")), false);
        assert_eq!(part2_password_checker((2, 9, 'c', "ccccccccc")), false);
    }
    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("1-5 m: zmmmsm"), (1, 5, 'm', "zmmmsm"));
        assert_eq!(parse_line("1-3 a: abcde"), (1, 3, 'a', "abcde"));
        assert_eq!(parse_line("1-3 b: cdefg"), (1, 3, 'b', "cdefg"));
    }
}
/* LEARNINGS: 
- :? debugging for println
    println!("bounds are: {:?}", bounds);

*/