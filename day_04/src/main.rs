// --- Day 4: Ceres Search ---

// XMAS: horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:

// ..X...
// .SAMX.
// .A..A.
// XMAS.S
// .X....

// The actual word search will be full of letters instead. For example:

// MMMSXXMASM
// MSAMXMSMSA
// AMXSXMAAMM
// MSAMASMSMX
// XMASAMXAMM
// XXAMMXXAMA
// SMSMSASXSS
// SAXAMASAAA
// MAMMMXMMMM
// MXMXAXMASX

// In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:

// ....XXMAS.
// .SAMXMS...
// ...S..A...
// ..A.A.MS.X
// XMASAMX.MM
// X.....XA.A
// S.S.S.S.SS
// .A.A.A.A.A
// ..M.M.M.MM
// .X.X.XMASX

// Take a look at the little Elf's word search. How many times does XMAS appear?
mod utils;

use utils::{backwards, diagonal, horizontal, parser, vertical};

fn main() {
    // parse input
    let input = parser::parse_input_to_string("src/input/input.txt").expect("error parsing input");

    //find horizontal
    let num_horizontal = horizontal::find_horizontal(&input);

    //find backwards
    let num_backwards = backwards::find_backwards(&input);

    //find vertical
    let num_vertical = vertical::find_vertical(&input);
    
    //find diagonal
    let num_diagonal = diagonal::find_diagonal(&input);
    
    let total = num_horizontal + num_backwards + num_vertical + num_diagonal;
    println!("Total: {}", total);
}
