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

pub fn find_grid_dimensions(file_path: &str) -> Result<(i32, i32), AntiNodeError> {
    let input = fs::read_to_string(file_path)?;
    let line_count: i32 = input.lines().count() as i32;
    let first_line = input.lines().next().ok_or(AntiNodeError::EmptyFile)?;
    let letters_per_line: i32 = first_line.chars().count() as i32;
    Ok((line_count, letters_per_line))
}
