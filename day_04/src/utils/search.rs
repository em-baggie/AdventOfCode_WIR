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

pub fn find_xmas_1(input: &Vec<Vec<char>>) -> Result<usize, FindXmasError> {
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

/// TODO
// pub fn find_xmas_2(input: &Vec<Vec<char>>) -> Result<usize, FindXmasError> {
    
// }

// tests for find_xmas_1
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

    assert_eq!(find_xmas_1(&input).unwrap(), 18);
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

    assert_eq!(find_xmas_1(&input).unwrap(), 0); 
}

// The Elf looks quizzically at you. Did you misunderstand the assignment?

// Looking for the instructions, you flip over the word search to find that this isn't actually an XMAS puzzle; it's an X-MAS puzzle in which you're supposed to find two MAS in the shape of an X. One way to achieve that is like this:

// M.S
// .A.
// M.S

// Irrelevant characters have again been replaced with . in the above diagram. Within the X, each MAS can be written forwards or backwards.

// Here's the same example from before, but this time all of the X-MASes have been kept instead:

// .M.S......
// ..A..MSMS.
// .M.S.MAA..
// ..A.ASMSM.
// .M.S.M....
// ..........
// S.S.S.S.S.
// .A.A.A.A..
// M.M.M.M.M.
// ..........

// In this example, an X-MAS appears 9 times.

// Flip the word search from the instructions back over to the word search side and try again. How many times does an X-MAS appear?
