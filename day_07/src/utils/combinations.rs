use itertools::Itertools;
use crate::utils::errors::BridgeRepairError;
use std::iter::repeat;

pub fn find_combinations(num_values: usize) -> Result<Vec<Vec<String>>, BridgeRepairError> {
    let num_combinations: usize = num_values - 1;
    let operators = vec!["*", "+"];
    let combinations = repeat(operators)
        .take(num_combinations as usize)
        .multi_cartesian_product()
        .map(|v| v.into_iter().map(|s| s.to_string()).collect())
        .collect();

    Ok(combinations)
}
