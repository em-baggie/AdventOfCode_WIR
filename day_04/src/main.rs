mod utils;

use utils::{ parser, search };

fn main() {
    // parse input
    let input_vec = parser::parse_input_to_vec("src/input/input.txt").expect("error parsing input");

    // find xmas matches in input
    let matches = search::find_xmas_1(&input_vec).expect("error finding xmas");
    println!("Number of XMAS matches: {}", matches);

}
