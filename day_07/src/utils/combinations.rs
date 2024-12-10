use itertools::Itertools;
use crate::utils::errors::BridgeRepairError;
use std::iter::repeat;

pub fn combinations(key: isize, values: Vec<isize>) -> Result<Vec<Vec<String>>, BridgeRepairError> {
    let num_combinations = values.len() - 1;
    let operators = vec!["*", "+"];
    let combinations = repeat(operators)
        .take(num_combinations)
        .multi_cartesian_product()
        .map(|v| v.into_iter().map(|s| s.to_string()).collect())
        .collect();

    return Ok(combinations);
    // need to find all of the possible combinations of + and * which does not have to include both of them

}

// 1 --> * or +
// 2 --> * * or + + or + * or * +