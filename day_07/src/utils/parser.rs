use crate::utils::errors::BridgeRepairError;
use std::collections::HashMap;
use std::fs;
use regex::Regex;
use std::sync::LazyLock;

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d+): (.+)").expect("Unable to create regex")
});

pub fn parse_input(file_path: &str) -> Result<HashMap<isize, Vec<isize>>, BridgeRepairError> {
    // define input and regex
    let input = fs::read_to_string(file_path)?;
    let re = &REGEX;

    // define empty vec for result
    let mut result = HashMap::new();

    // iterate over lines in input
    for line in input.lines() {
        for cap in re.captures_iter(line) {
            let key: isize = cap[1].parse()?;
            let value: Vec<isize> = cap[2]
            .split_whitespace()
            .map(|x| x.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?;

            result.insert(key, value);
        };
    }
    Ok(result)
}

