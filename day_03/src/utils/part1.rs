use regex::Regex;

pub fn mul_results_part_1(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Invalid regex");
    let mut total: usize = 0;

    re.captures_iter(input).for_each(|cap| {
        let x: usize = cap[1].parse().expect("Unable to parse");
        let y: usize = cap[2].parse().expect("Unable to parse");
        total += x * y;
    });

    total
}