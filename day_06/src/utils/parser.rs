use std::fs;
use crate::utils::errors::GuardGallivantError;

pub fn parse_input_to_vec(file_path: &str) -> Result<Vec<Vec<char>>, GuardGallivantError> { 
    let file = fs::read_to_string(file_path)?;
    let output: Vec<Vec<char>> = file.split("\n").map(|s| s.chars().collect()).collect();

    let row_length = output.get(0).map_or(0, |row| row.len());
    if !output.iter().all(|row| row.len() == row_length) {
        return Err(GuardGallivantError::ParseError("Uneven row and columns lengths".to_string()));
    }
    Ok(output)
}