use std::fs;
use regex::Regex;

fn parse_input(file_path: &str) -> Result<String, std::io::Error> { 
    let file = fs::read_to_string(file_path).unwrap(); 
    Ok(file)
}

fn part_1(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Invalid regex");
    let mut total: usize = 0;

    re.captures_iter(input).for_each(|cap| {
        let x: usize = cap[1].parse().expect("Unable to parse");
        let y: usize = cap[2].parse().expect("Unable to parse");
        total += x * y;
    });

    total
}

fn main() {
    let input = parse_input("input.txt").unwrap();

    // part 1
    let result = part_1(&input);
    println!("{}", result);

    // part 2
    
}
