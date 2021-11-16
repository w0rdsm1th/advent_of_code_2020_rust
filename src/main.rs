mod utils;
mod day7;


fn main() {
    let day_number = '7';
    let puzzle_input = utils::input_reader(day_number);
    day7::part1(&puzzle_input);
    // day7::part2(&puzzle_input);
    // day5::part2(puzzle_input.lines().collect());
    // day4::part2(puzzle_input.lines().map(|n| n.trim()).collect());
}
