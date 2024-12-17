use std::collections::{HashMap, HashSet};
use crate::utils::errors::AntiNodeError;

pub fn antinode(map: HashMap<char, Vec<(usize, usize)>>, dimensions: (usize, usize)) -> Result(usize, AntiNodeError) {
    let mut unique_antinodes = HashSet::new();
    for coords in map.values() {
        let num_coords = coords.len();
        for i in 0..num_coords {
            for j in 1..num_coords {
                let diff: (i32, i32) = (
                    (coords[i].0 as i32 - coords[j].0 as i32).abs(),
                    (coords[i].1 as i32 - coords[j].1 as i32).abs(),
                );



            }
        }
    }
    // for each char in hashmap, check combinations and whether antinodes would be in grid
    // if would be in grid, add to hashset
    // calculate number of values in hashset and return

    Ok(1)
}
