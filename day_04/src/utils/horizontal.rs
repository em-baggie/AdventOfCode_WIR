use regex::Regex;

pub fn find_horizontal(input: &str) -> usize {
    let re = Regex::new(r"XMAS").expect("Invalid regex");
    let mut total: usize = 0;
    
    total += re.captures_iter(input).count();

    total
}

#[test]
fn test_find_horizontal() {
    let input = "XMASXMASXMAS";
    assert_eq!(find_horizontal(&input), 3);
}

#[test]
fn test_find_horizontal_empty() {
    let input = "";
    assert_eq!(find_horizontal(&input), 0);
}