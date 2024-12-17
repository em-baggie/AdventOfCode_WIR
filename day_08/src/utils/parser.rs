use std::collections::HashMap;
use crate::utils::errors::AntiNodeError;
use std::fs;


pub fn parse_input(file_path: &str) -> Result<HashMap<char, Vec<(usize, usize)>>, AntiNodeError> {
    // define input and regex
    let input = fs::read_to_string(file_path)?;
    let mut result = HashMap::<char, Vec<(usize, usize)>>::new();

    for line in input.lines().enumerate() {
        for char in line.1.chars().enumerate() {
            if char.1 != '.' {
                result.entry(char.1)
                    .or_insert_with(Vec::new)
                    .push((line.0, char.0));
            }
        }
    }
    Ok(result)
}

pub fn find_grid_dimensions(file_path: &str) -> Result<(usize, usize), AntiNodeError> {
    let input = fs::read_to_string(file_path)?;
    let line_count = input.lines().count();
    let first_line = input.lines().next().ok_or(AntiNodeError::EmptyFile)?;
    let letters_per_line: usize = first_line.chars().count();
    Ok((line_count - 1, letters_per_line - 1))
}
