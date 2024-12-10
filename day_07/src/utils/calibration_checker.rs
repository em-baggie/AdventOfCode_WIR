use std::collections::HashMap;
use crate::utils::combinations;
use crate::utils::errors::BridgeRepairError;

pub fn check_calibration(map: HashMap<i64, Vec<i64>>) -> Result<i64, BridgeRepairError> {
    let mut total: i64 = 0;

    // iterate over each key, value pair in the map
    for (key, value) in map.into_iter() {
        // Ensure all values in the vector are i64
        let value: Vec<i64> = value.into_iter().map(|v| v as i64).collect();
        // find all possible combinations of operators for the value
        let operator_combinations = combinations::find_combinations(value.len())?;

        // iterate over each combination of operators
        for combination in operator_combinations {
            let mut sum: i64 = value[0];
            // evaluate the total for each combination of operators
            for (index, operator) in combination.into_iter().enumerate() {
                sum = match operator.as_str() {
                    "+" => sum.checked_add(value[index + 1]).ok_or(BridgeRepairError::CheckCalibrationError("Overflow occurred"))?,
                    "*" => sum.checked_mul(value[index + 1]).ok_or(BridgeRepairError::CheckCalibrationError("Overflow occurred"))?,
                    _ => return Err(BridgeRepairError::CheckCalibrationError("Unable to match operator")),
                };
            }
            
            // if the sum matches the key, add the key to the total and break
            if sum == key {
                println!("{:?}", key);
                total += key;
                break; // Move onto the next key, value pair
            }
        }
    }
    Ok(total)
}
