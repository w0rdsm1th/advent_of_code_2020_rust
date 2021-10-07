mod utils;
mod day5;


fn main() {
    let day_number = '5';
    let puzzle_input = utils::input_reader(day_number);
    day5::part1(puzzle_input.lines().collect());
    // day5::part2(&puzzle_input);
    // day4::part2(puzzle_input.lines().map(|n| n.trim()).collect());
    // println!("Hello, world!");
}
