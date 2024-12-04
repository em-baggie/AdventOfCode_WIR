use regex::Regex;

pub fn mul_results_part_2(input: &str) -> usize {
    let re = Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").expect("Invalid regex");
    let mut total: usize = 0;
    let mut skip = false;

    re.captures_iter(input)
        .for_each(|cap| {
            if &cap[1] == "don't()" {
                skip = true; 
            } else if &cap[1] == "do()" {
                skip = false;
            } else if !skip {
                let x: usize = cap[2].parse().expect("Unable to parse");
                let y: usize = cap[3].parse().expect("Unable to parse");
                total += x * y;
            }
        });
    total
}