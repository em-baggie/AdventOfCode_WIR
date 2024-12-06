// X|Y means that if both page number X and page number Y are to be produced as part of an update, page number X must be printed at some point before page number Y.

// test_rules.txt and test_updates1.txt
// 3 updates are in the correct order

// test_updates2.txt
// 61 + 53 + 29 = 143

// find how many updates are in correct order
// find the middle page number of each correctly-ordered update and add them together

use std::error::Error;
mod utils;
use utils::{ parser, check_updates, errors, find_middle };

fn main() -> Result<(), Box<dyn Error>> {
    let rules = parser::parse_rules("src/input/input_rules.txt")?;
    println!("Rules: {:?}", rules);

    let updates = parser::parse_updates("src/input/input_updates.txt")?;
    println!("Updates: {:?}", updates);

    let sorted_updates = check_updates::check_updates(&rules, &updates)?;
    println!("Sorted updates: {:?}", sorted_updates);
    println!("Number of updates in correct order: {}", sorted_updates.len());

    let sum_of_middle_pages = find_middle::sum_of_middle_pages(&sorted_updates)?;

    println!("Sum of middle pages: {}", sum_of_middle_pages);

    Ok(())
}
