mod utils;
mod day8;


fn main() {
    let day_number = '8';
    let puzzle_input = utils::input_reader(day_number);
    day8::part2(&puzzle_input);
}
