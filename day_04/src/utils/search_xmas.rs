use crate::utils::errors::FindXmasError;

/// Searches 2D grid of characters for the target string "XMAS" by checking all possible directions from each position in the grid
///
/// # Arguments
///
/// * `input` - vector of vectors of characters from the input file
///
/// # Returns
///
/// * `Result<i32, FindXmasError>` - The number of times XMAS appears in the input, or an error

pub fn find_xmas(input: &Vec<Vec<char>>) -> Result<usize, FindXmasError> {
    let num_cols = input[0].len();
    let num_rows = input.len();
    let target:Vec<_> = "XMAS".chars().collect();
    let dirs: Vec<(isize, isize)> = vec![(1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1)];
    let mut count = 0;

    for r in 0..num_rows {
        for c in 0..num_cols {
            for dir in &dirs {
                let mut pos = (r as isize, c as isize);
                let mut matched_letters = 0;
                for i in 0..target.len() {
                    if pos.0 >= 0 && pos.0 < num_rows as isize && pos.1 >= 0 && pos.1 < num_cols as isize {
                        if input[pos.0 as usize][pos.1 as usize] == target[i] {
                            matched_letters += 1;
                        }
                    }
                    pos.0 += dir.0;
                    pos.1 += dir.1;
                }
                if matched_letters == 4 {
                    count += 1;
                }
            }
        }
    }
    Ok(count)
}

// tests for find_xmas
#[test]
fn test_find_matches() {
    let input = vec![
        vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
        vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
        vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
        vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
        vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
        vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
        vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
        vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
        vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
        vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
    ];

    assert_eq!(find_xmas(&input).unwrap(), 18);
}

#[test]
fn test_find_no_matches() {
    let input = vec![
        vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
    ];

    assert_eq!(find_xmas(&input).unwrap(), 0); 
}