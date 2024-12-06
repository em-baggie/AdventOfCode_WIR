use crate::errors::CheckUpdatesError;
use crate::utils::parser;
use std::error::Error;

/// Finds the sum of the middle page numbers of each update
/// Arguments:
///     updates: A vector of vectors, where each inner vector represents an update
/// Returns:
///     The sum of the middle page numbers of each update, or a CheckUpdatesError

pub fn sum_of_middle_pages(updates: &Vec<Vec<i32>>) -> Result<i32, CheckUpdatesError> {
    let mut sum = 0;
    for update in updates {
        let len = update.len();
        let middle = update[len / 2];
        sum += middle;
    }
    Ok(sum)
}

#[test]
fn test_sum_of_middle_pages() -> Result<(), Box<dyn Error>> {
    let updates = parser::parse_updates("src/input/test_updates2.txt")?;
    let sum = sum_of_middle_pages(&updates)?;
    assert_eq!(sum, 143);
    Ok(())
}