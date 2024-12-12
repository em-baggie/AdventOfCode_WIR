use crate::utils::combinations;
use crate::utils::errors::BridgeRepairError;

pub fn check_calibration(map: Vec<(i64, Vec<i64>)>) -> Result<i64, BridgeRepairError> {
    let mut total: i64 = 0;

    // iterate over each key, value pair in the map
    for (key, value) in map.into_iter() {
        // find all possible combinations of operators for the value
        let operator_combinations: Vec<Vec<String>> = combinations::find_combinations(value.len())?;

        // iterate over each combination of operators
        for combination in operator_combinations {
            // if key == 894 {
            //     println!("Combination: {:?}", combination);
            // }
            let mut sum: i64 = value[0];
            // evaluate the total for each combination of operators
            for (index, operator) in combination.into_iter().enumerate() {
                sum = match operator.as_str() {
                    "+" => sum.checked_add(value[index + 1]).ok_or(BridgeRepairError::CheckCalibrationError("Overflow occurred"))?,
                    "*" => sum.checked_mul(value[index + 1]).ok_or(BridgeRepairError::CheckCalibrationError("Overflow occurred"))?,
                    _ => return Err(BridgeRepairError::CheckCalibrationError("Unable to match operator")),
                };
            }
            
            // if key == 894 {
            //     println!("Key: {}, Sum: {}", key, sum);
            // }

            // if the sum matches the key, add the key to the total and break
            if sum == key {
                total += key;
                println!("{}", key);
                break; // Move onto the next key, value pair
            }


        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parser;
    use crate::utils::errors::BridgeRepairError;

    #[test]
    fn test_check_calibration() -> Result<(), BridgeRepairError> {
        let input = parser::parse_input("src/input/input_test.txt")?;
        let result = check_calibration(input)?;
        assert_eq!(result, 3749);
        Ok(())
    }
}