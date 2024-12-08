use std::fs;
use crate::utils::errors::GuardGallivantError;
/// Parses the input file into a 2D vector of characters.
/// 
/// # Arguments
/// 
/// * `file_path` - The path to the input file.
/// 
/// # Returns
/// 
/// A Result containing a 2D vector of characters or a GuardGallivantError if the file cannot be read or the input is invalid.

pub fn parse_input_to_vec(file_path: &str) -> Result<Vec<Vec<char>>, GuardGallivantError> { 
    let file = fs::read_to_string(file_path)?;
    let output: Vec<Vec<char>> = file.split("\n").map(|s| s.chars().collect()).collect();

    let row_length = output.get(0).map_or(0, |row| row.len());
    if !output.iter().all(|row| row.len() == row_length) {
        return Err(GuardGallivantError::ParseError("Uneven row and columns lengths".to_string()));
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_parse_input_to_vec_test_error() -> Result<(), Box<dyn Error>> {
        let file_path = "src/input/testinput1.txt";
        let result = parse_input_to_vec(file_path);
        assert!(matches!(result, Err(GuardGallivantError::ParseError(_))));
        Ok(())
    }
}