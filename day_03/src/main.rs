mod utils;

use utils::{parser, part1, part2};

fn main() {
    // parse input
    let input = parser::parse_input("src/input/input.txt").expect("Failed to parse input");

    // part 1
    let result = part1::mul_results_part_1(&input);
    println!("{}", result);

    // part 2
    let result = part2::mul_results_part_2(&input);
    println!("{}", result)
}