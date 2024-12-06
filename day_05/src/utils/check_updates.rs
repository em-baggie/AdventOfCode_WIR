use crate::errors::CheckUpdatesError;
use std::collections::HashMap;
use std::error::Error;
use crate::parser;

/// Checks if the updates are in the correct order according to the rules
/// Arguments:
///     rules: A hashmap, where the key is the rule number and the value is a vector of numbers the key must precede in the update
///     updates: A vector of vectors, where each inner vector represents an update
/// Returns:
///     A vector of vectors, where each inner vector represents an update that follows all of the rules, or a CheckUpdatesError

pub fn check_updates(rules: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) -> Result<Vec<Vec<i32>>, CheckUpdatesError> {
    let mut updates_in_order: Vec<i32> = vec![0; updates.len()];
    // for each update
    for (update_index, update) in updates.iter().enumerate() {
        // for each key value pair in rules
        for rule in rules {
            // get the key
            let key = *rule.0;
            // check if the key is in the update
            if update.contains(&key) {
                // find index of the value of the key in the update
                let Some(index) = update.iter().position(|&r| r == key) else {
                    return Err(CheckUpdatesError::CheckUpdateError("Unable to find key in update".to_string()));
                };
                let is_update_in_order = update.iter().take(index).all(|&num| !rule.1.contains(&num));
                if !is_update_in_order {
                    updates_in_order[update_index] = 1;
                }
            }
        }
    }
    
    let filtered_updates: Vec<Vec<i32>> = updates.iter().enumerate()
        .filter(|(i, _)| updates_in_order[*i] != 1)
        .map(|(_, update)| update.clone())
        .collect();
    Ok(filtered_updates)
}

#[test]
fn test_check_updates() -> Result<(), Box<dyn Error>> {
    let rules = parser::parse_rules("src/input/test_rules.txt")?;
    let updates = parser::parse_updates("src/input/test_updates1.txt")?;
    let sorted_updates = check_updates(&rules, &updates)?;
    assert_eq!(sorted_updates.len(), 3);
    assert_eq!(sorted_updates[0], vec![75, 47, 61, 53, 29]);
    assert_eq!(sorted_updates[1], vec![97, 61, 53, 29, 13]);
    assert_eq!(sorted_updates[2], vec![75, 29, 13]);
    Ok(())
}