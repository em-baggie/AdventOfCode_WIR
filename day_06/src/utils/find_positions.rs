use crate::utils::directions::Direction;
use crate::utils::errors::GuardGallivantError;

pub fn find_num_positions(input: &Vec<Vec<char>>) -> Result<usize, GuardGallivantError> {
    // find start position and direction
    let start_coord_dir: (usize, usize, char) = input.iter().enumerate()
        .find_map(|(r, row)| {
            row.iter().enumerate().find(|(_, &c)| c == '>' || c == '<' || c == '^' || c == 'v')
                .map(|(c, &ch)| (c, r, ch))
        })
        .ok_or(GuardGallivantError::FindPositionsError("No start position found".to_string()))?;

    // initialisation
    let mut count = 0;
    let mut coord = (start_coord_dir.0, start_coord_dir.1);
    let mut direction = match start_coord_dir.2 {
        '>' => Direction::Right,
        '<' => Direction::Left,
        '^' => Direction::Up,
        'v' => Direction::Down,
        _ => return Err(GuardGallivantError::FindPositionsError("Invalid direction".to_string())),
    };
    let num_cols = input[0].len();
    let num_rows = input.len();

    while coord.0 < num_cols && coord.1 < num_rows {
        // track path 
        // and count distinct positions before leaving map, use windows method to check if next char is < and direction changes
    }
    Ok(count)
}
