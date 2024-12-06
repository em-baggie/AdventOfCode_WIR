use std::fs;
use crate::utils::errors::FindXmasError;

/// Reads a file and converts its contents into a 2D vector of characters with rows and columns
///
/// # Arguments
///
/// * `file_path` - path to the input file
///
/// # Returns
///
/// * `Result<Vec<Vec<char>>, FindXmasError>` - a 2D vector of characters or an error
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read
/// - The vector of vectors contains rows of different lengths

pub fn parse_input_to_vec(file_path: &str) -> Result<Vec<Vec<char>>, FindXmasError> { 
    let file = fs::read_to_string(file_path)?;
    let output: Vec<Vec<char>> = file.split("\n").map(|s| s.chars().collect()).collect();

    let row_length = output.get(0).map_or(0, |row| row.len());
    if !output.iter().all(|row| row.len() == row_length) {
        return Err(FindXmasError::from_shape_error("Uneven row and columns lengths".to_string()));
    }
    Ok(output)
}

#[test]
fn test_parse_input_to_vec() {
    let result = parse_input_to_vec("./src/input/input1.txt");
    assert!(result.is_err());
    if let Err(e) = result {
        match e {
            FindXmasError::VecCreationError(_) => assert!(true),
            _ => assert!(false, "Expected VecCreationError, but got a different error."),
        }
    }
}