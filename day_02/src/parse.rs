use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

pub fn parse_input(file_name: &str) -> Result<Vec<Vec<i32>>, std::io::Error> { 
    let mut reports = Vec::new();
    let f = File::open(file_name)?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line?;
        let nums: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        reports.push(nums);
    }
    Ok(reports)
}
