use std::fs;

fn parse_input(file_path: &str) -> Result<(Vec<i32>, Vec<i32>), std::io::Error> { 
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    let file = fs::read_to_string(file_path).unwrap(); 

    for line in file.lines() {
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        if let (Ok(first), Ok(second)) = (line_parts[0].parse::<i32>(), line_parts[1].parse::<i32>()) {
            list1.push(first);
            list2.push(second);
        }
    }
    Ok((list1, list2))
} 

// solution to part 1
fn advent1a(mut list1: Vec<i32>, mut list2: Vec<i32>) -> i32 {
    list1.sort();
    list2.sort();

    list1.into_iter().zip(list2.into_iter()).map(|(a, b)| (a - b).abs()).sum()
}

// solution to part 2 (total similarity score)
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

    // change this to advent1b for part 2
    let result = advent1a(list1, list2);
    println!("{}", result);
}
