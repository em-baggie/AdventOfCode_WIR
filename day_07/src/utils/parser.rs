use crate::utils::errors::BridgeRepairError;
use std::collections::HashMap;
use std::fs;
use regex::Regex;
use std::sync::LazyLock;

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d+): (\d+( \d+)+)").expect("Unable to create regex")
});


pub fn parse_input(file_path: &str) -> Result<Vec<HashMap<i32, Vec<i32>>>, BridgeRepairError> {
    let input = fs::read_to_string(file_path)?;
    let re = REGEX;
    for line in input.lines() {
        for cap in re.captures_iter(&input) {
            let mut map = HashMap::new();
            let result: usize = cap[1].parse()?;
            for i in 2..cap.len() {
                let value = cap[i].parse()?;
                map.entry(result).or_insert_with(Vec::new).push(value);
            }
        };
        
    }
    Ok(result)
}

