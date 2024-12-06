mod utils;

use utils::{ parser, search_xmas };

fn main() {
    // parse input
    let input_vec = parser::parse_input_to_vec("src/input/input.txt").expect("error parsing input");

    // find xmas matches in input
    let matches = search_xmas::find_xmas(&input_vec).expect("error finding xmas");
    println!("num_diagonal: {}", matches);

}
