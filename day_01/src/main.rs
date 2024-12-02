// 2 lists 
// find diff between smallest - largest numbers and add up

// For example:

// 3   4
// 4   3
// 2   5
// 1   3
// 3   9
// 3   3

// In the example list above, the pairs and distances would be as follows:

// Your actual left and right lists contain many location IDs. What is the total distance between your lists?

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn parse_input(file_name: &str) -> Result<(Vec<i32>, Vec<i32>), std::io::Error> { 
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    let f = File::open(file_name)?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line?;
        let mut line_parts = line.split_whitespace();

        // use pattern matching instead of creating a new vec each time:
        if let (Some(first_str), Some(second_str)) = (line_parts.next(), line_parts.next()) {
            let first_num = first_str.parse::<i32>().unwrap(); 
            let second_num = second_str.parse::<i32>().unwrap();

            list1.push(first_num);
            list2.push(second_num);
        
        }
    }
    Ok((list1, list2))
} 

fn advent1(mut list1: Vec<i32>, mut list2: Vec<i32>) -> i32 {
    list1.sort();
    list2.sort();
    
    let iter1 = list1.into_iter();
    let iter2 = list2.into_iter();

    iter1.zip(iter2).map(|(a, b)| (a - b).abs()).sum()
    }

fn main() {
    let lists = parse_input("advent_1_input.txt").unwrap();
    let list1 = lists.0;
    let list2 = lists.1;

    let result = advent1(list1, list2);
    println!("{}", result);
}
