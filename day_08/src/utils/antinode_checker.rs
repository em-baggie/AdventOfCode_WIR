use std::collections::{HashMap, HashSet};
use crate::utils::errors::AntiNodeError;

pub fn antinode(map: HashMap<char, Vec<(usize, usize)>>, dimensions: (i32, i32)) -> Result<usize, AntiNodeError> {

    fn is_within_bounds(coord: (i32, i32), dimensions: (i32, i32)) -> bool {
        coord.0 >= 0 && coord.0 < dimensions.0 && coord.1 >= 0 && coord.1 < dimensions.1
    }

    let mut unique_antinodes = HashSet::new();
    for coords in map.values() {
        let num_coords = coords.len();
        for i in 0..num_coords {
            for j in i + 1..num_coords {
                let diff: (i32, i32) = (
                    coords[j].0 as i32 - coords[i].0 as i32,
                    coords[j].1 as i32 - coords[i].1 as i32,
                );
                let new_coord1: (i32, i32) = (coords[i].0 as i32 - diff.0, coords[i].1 as i32 - diff.1);
                let new_coord2: (i32, i32) = (coords[j].0 as i32 + diff.0, coords[j].1 as i32 + diff.1);


                if is_within_bounds(new_coord1, dimensions) {
                    unique_antinodes.insert(new_coord1);
                }

                if is_within_bounds(new_coord2, dimensions) {
                    unique_antinodes.insert(new_coord2);
                }

            }
        }
    }
    Ok(unique_antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{parser, antinode_checker};
    #[test]
    fn test_antinode() -> Result<(), AntiNodeError>  {
        let input = parser::parse_input("src/input/input_test.txt")?;
        let dimensions = parser::find_grid_dimensions("src/input/input_test.txt")?;
        let antinodes = antinode_checker::antinode(input, dimensions)?;
        assert_eq!(antinodes, 14);
        Ok(())
    }
}
