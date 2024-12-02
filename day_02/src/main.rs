// The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a list of numbers called levels that are separated by spaces. For example:

// 7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9

// 2 reports are safe from above

// cols are reports, rows contain levels

// check if all are increasing or all are decreasing
// any two levels differ by at least one and at most three

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn parse_input(file_name: &str) -> Result<Vec<Vec<i32>>, std::io::Error> { 
    let mut reports = Vec::new();

    let f = File::open(file_name)?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line?;
        let line_parts: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        
        reports.push(line_parts);
    }
    Ok(reports)
}

fn check(report: &Vec<i32>) -> bool {
    let mut decr = false;
    let mut incr = false;

    if report[0] > report[1] {
        decr = true;
    } else if report[0] < report[1] {
        incr = true;
    } else {
        return false;
    };

    for i in 0..(report.len() - 1) {
        let diff; // Declare diff outside the if-else

        if incr {
            diff = report[i + 1] - report[i]; // Corrected the order for increasing
        } else {
            diff = report[i] - report[i + 1]; // Corrected the order for decreasing
        }

        if diff > 3 || diff < 1 {
            return false;
        }
    }
    true
}

fn check_reports(reports: Vec<Vec<i32>>) -> usize {
    reports.into_iter()
    .filter(|x| check(x))
    .count()
}

fn main() {
    let input1 = parse_input("input.txt").unwrap();
    println!("{}", check_reports(input1));
}


