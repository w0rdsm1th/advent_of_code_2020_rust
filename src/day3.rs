/*
project: advent_of_code_2020_rust
created by: aleks
on: 21 June 2021

considerations:
    - tiling/repeating of the pattern to the right
    >> can handle 'falling off the edge'

    - hitting a tree, then increments the counter


*/

fn check_tree(inp: &char) -> bool {
    inp == &'#'
}

pub fn part1(puzzle_input: &Vec<&str>, x_slope: usize, y_slope: usize) -> u32 {
    /*
    starting position 0, 0
    and slope right 3, down 1

    return count of trees hit
    */
    let mut cur_x_pos = 0_usize;
    let mut cur_y_pos = 0_usize;
    let height = puzzle_input.len() as usize;
    let width = puzzle_input[0].len() as usize;
    let mut counter = 0;
    let mut loop_counter = 0;

    // until hit the bottom of the puzzle input
    loop {
        let exceed_width = if (cur_x_pos + x_slope >= width) { 1 } else { 0 };

        // if exceed the width of the input, then handle
        cur_x_pos = cur_x_pos + x_slope - width * exceed_width;
        cur_y_pos = cur_y_pos + y_slope;

        // println!("cur_pos: {:?}", (cur_x_pos, cur_y_pos));
        if cur_y_pos + y_slope > height { break; }

        if (puzzle_input[cur_y_pos].as_bytes()[cur_x_pos] as char == '#') {
            counter = counter + 1
        }

        loop_counter = loop_counter + 1;
        if loop_counter > 1_000 {
            panic!("loop counter breakout");
        }
    }
    println!("counter {:?}", &counter);
    counter
}


pub fn part2(puzzle_input: Vec<&str>) -> u64 {
    /* try for a few different candidate slopes and multiply them together */
    let part2_slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let mut trees_counter: u64 = 1;
    for slope in part2_slopes {
        trees_counter *= part1(&puzzle_input, slope.0, slope.1) as u64
    }

    println!("trees_counter {:?}", &trees_counter);
    trees_counter
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1_sample() {
        let sample = r#"..##.......
                    #...#...#..
                    .#....#..#.
                    ..#.#...#.#
                    .#...##..#.
                    ..#.##.....
                    .#.#.#....#
                    .#........#
                    #.##...#...
                    #...##....#
                    .#..#...#.#"#;
        assert_eq!(part1(&sample.lines().map(|n| n.trim()).collect(), 3, 1), 7);
    }

    #[test]
    fn test_part2_sample() {
        let sample = r#"..##.......
                    #...#...#..
                    .#....#..#.
                    ..#.#...#.#
                    .#...##..#.
                    ..#.##.....
                    .#.#.#....#
                    .#........#
                    #.##...#...
                    #...##....#
                    .#..#...#.#"#;
        assert_eq!(part2(sample.lines().map(|n| n.trim()).collect()), 336);
    }
}
/* LEARNINGS: 
-
*/