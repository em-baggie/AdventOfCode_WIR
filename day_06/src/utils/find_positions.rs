use crate::utils::directions::Direction;
use crate::utils::errors::GuardGallivantError;

pub fn find_num_positions(mut input: Vec<Vec<char>>) -> Result<usize, GuardGallivantError> {
    println!("finding num positions");
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
    println!("num_cols: {}", num_cols);
    let num_rows = input.len() as isize;
    println!("num_rows: {}", num_rows);

    // check current position is not out of bounds
    let mut iterations = 0;
    loop {
        input[coord.1 as usize][coord.0 as usize] = 'X';
        let mut next_x = coord.0 + offset.0;
        let mut next_y = coord.1 + offset.1;
        while next_x < num_cols as isize && next_y < num_rows as isize && input[next_y as usize][next_x as usize] == '#' {
            direction = direction.change_direction();
            offset = direction.coordinate_offset();
            next_x = next_x as isize + offset.0;
            next_y = next_y as isize + offset.1;

            iterations += 1;
            if iterations > 3000 {
                return Err(GuardGallivantError::FindPositionsError("Too many iterations, possible infinite loop".to_string()));
            }
        }

        if next_x < num_cols && next_y < num_rows {
            coord = (next_y, next_x);
        } else {
            break;
        }
        println!("coord: {:?}", coord);
    }
    let count = input.iter().flatten().filter(|c| **c == 'X').count();

    Ok(count)
}
