// This time, you'll need to figure out exactly how often each number from the left list appears in the right list. Calculate a total similarity score by adding up each number in the left list after multiplying it by the number of times that number appears in the right list.

// Here are the same example lists again:

// 3   4
// 4   3
// 2   5
// 1   3
// 3   9
// 3   3

// For these example lists, here is the process of finding the similarity score:

//     The first number in the left list is 3. It appears in the right list three times, so the similarity score increases by 3 * 3 = 9.
//     The second number in the left list is 4. It appears in the right list once, so the similarity score increases by 4 * 1 = 4.
//     The third number in the left list is 2. It does not appear in the right list, so the similarity score does not increase (2 * 0 = 0).
//     The fourth number, 1, also does not appear in the right list.
//     The fifth number, 3, appears in the right list three times; the similarity score increases by 9.
//     The last number, 3, appears in the right list three times; the similarity score again increases by 9.

// So, for these example lists, the similarity score at the end of this process is 31 (9 + 4 + 0 + 0 + 9 + 9).

// Once again consider your left and right lists. What is their similarity score?

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

fn advent1a(mut list1: Vec<i32>, mut list2: Vec<i32>) -> i32 {
    list1.sort();
    list2.sort();
    
    let iter1 = list1.into_iter();
    let iter2 = list2.into_iter();

    iter1.zip(iter2).map(|(a, b)| (a - b).abs()).sum()
    }

fn advent1b(list1: Vec<i32>, list2: Vec<i32>) -> i32 {

    list1
    .into_iter()
    .map(|x| x * (list2.iter().filter(|&&n| n == x))
    .count() as i32)
    .sum()
}

fn main() {
    let lists = parse_input("advent_1_input.txt").unwrap();
    let list1 = lists.0;
    let list2 = lists.1;

    let result = advent1b(list1, list2);
    println!("{}", result);
}
