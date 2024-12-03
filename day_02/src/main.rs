mod parse;
mod part_1;
mod part_2;

fn main() {
    // part 1 solution:
    let input1 = parse::parse_input("input.txt").unwrap();
    println!("{}", part_2::safe_reports(input1).0.len());

    // part 2 solution:
    
    
}


