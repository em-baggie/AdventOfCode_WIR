use crate::utils::errors::CheckUpdatesError;
use std::collections::HashMap;
use std::fs;

/// Parses the rules from a file to a hashmap
/// Arguments:
///     file_path: The path to the file containing the rules
/// Returns:
///     A hashmap, where the key is the number and the value is a vector of numbers the key must precede in the update, or a CheckUpdatesError

pub fn parse_rules(file_path: &str) -> Result<HashMap<i32, Vec<i32>>, CheckUpdatesError> {
    let mut map = HashMap::new();
    let file = fs::read_to_string(file_path)?;
    for line in file.lines() {
        let (key, value) = line.split_once('|').ok_or_else(|| CheckUpdatesError::CreateHashMapError("Invalid input format".to_string()))?;
        
        // Convert key and value from string to i32
        let key: i32 = key.parse().map_err(|_| CheckUpdatesError::CreateHashMapError("Invalid hashmap key".to_string()))?;
        let value: i32 = value.parse().map_err(|_| CheckUpdatesError::CreateHashMapError("Invalid hashmap value".to_string()))?;

        // Insert key-value pair into the hashmap
        map.entry(key).or_insert_with(Vec::new).push(value);
    }
    Ok(map)
}

/// Parses the updates from a file to a vector of vectors
/// Arguments:
///     file_path: The path to the file containing the updates
/// Returns:
///     A vector of vectors, where each inner vector represents an update, or a CheckUpdatesError


pub fn parse_updates(file_path: &str) -> Result<Vec<Vec<i32>>, CheckUpdatesError> {
    let file = fs::read_to_string(file_path)?;
    let updates: Vec<Vec<i32>> = file.lines().map(|line| line.split(',').map(|num| num.parse().unwrap()).collect()).collect();
    // if update is not odd length, return error
    for update in &updates {
        if update.len() % 2 == 0 {
            return Err(CheckUpdatesError::CreateHashMapError("All updates are not odd in length".to_string()));
        }
    }
    Ok(updates)
}