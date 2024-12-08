use crate::utils::directions::Direction;
use crate::utils::errors::GuardGallivantError;

/// Finds the number of distinct positions that the guard will 
/// 
/// # Arguments
/// 
/// * `input` - A 2D vector of characters representing the input grid
/// 
/// # Returns
/// 
/// The number of distinct positions or a GuardGallivantError if an error occurs

pub fn find_num_positions(mut input: Vec<Vec<char>>) -> Result<usize, GuardGallivantError> {
    // find start position and direction
    let start_coord_dir: (usize, usize, char) = input.iter().enumerate()
        .find_map(|(r, row)| {
            row.iter().enumerate().find(|(_, &c)| c == '>' || c == '<' || c == '^' || c == 'v')
                .map(|(c, &ch)| (c, r, ch))
        })
        .ok_or(GuardGallivantError::FindPositionsError("No start position found".to_string()))?;
    // initialisation
    let mut coord: (isize, isize) = (start_coord_dir.0 as isize, start_coord_dir.1 as isize);
    let mut direction = match start_coord_dir.2 {
        '>' => Direction::Right,
        '<' => Direction::Left,
        '^' => Direction::Up,
        'v' => Direction::Down,
        _ => return Err(GuardGallivantError::FindPositionsError("Invalid direction".to_string())),
    };
    let mut offset = direction.coordinate_offset();
    let num_cols = input[0].len() as isize;
    let num_rows = input.len() as isize;

    // check current position is not out of bounds
    loop {
        input[coord.1 as usize][coord.0 as usize] = 'X';
        let mut next_x = coord.0 + offset.0;
        let mut next_y = coord.1 + offset.1;
        while next_x < num_cols as isize && next_y < num_rows as isize && input[next_y as usize][next_x as usize] == '#' {
            direction = direction.change_direction();
            offset = direction.coordinate_offset();
            next_x = coord.0 + offset.0;
            next_y = coord.1 + offset.1;
        }

        if next_x < num_cols && next_y < num_rows {
            coord = (next_x, next_y);
        } else {
            break;
        }
    }
    let count = input.iter().flatten().filter(|c| **c == 'X').count();

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parser;
    use std::error::Error;

    #[test]
    fn test_find_num_positions() -> Result<(), Box<dyn Error>> {
        let input = parser::parse_input_to_vec("src/input/testinput.txt")?;
        let result = find_num_positions(input)?;
        assert_eq!(result, 41);
        Ok(())
    }

    #[test]
    fn test_find_num_positions_error() -> Result<(), Box<dyn Error>> {
        let input = parser::parse_input_to_vec("src/input/testinput2.txt")?;
        let result = find_num_positions(input);
        assert!(matches!(result, Err(GuardGallivantError::FindPositionsError(_))));
        Ok(())
    }
}   