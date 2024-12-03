use std::fs;

fn parse_input(file_name: &str) -> Result<Vec<Vec<i32>>, std::io::Error> { 
    let mut reports = Vec::new();
    let file = fs::read_to_string(file_name).unwrap();

    for line in file.lines() {
        let nums: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        reports.push(nums);
    }
    Ok(reports)
}

// part 1
fn report_is_safe(vec: &Vec<i32>) -> bool {
    (vec.windows(2).all(|w| w[0] < w[1]) || vec.windows(2).all(|w| (w[0] > w[1])))
    
    && vec.windows(2).all(|w| (w[0] - w[1]).abs() <= 3 && (w[0] - w[1]).abs() >= 1)
}

fn safe_reports(reports: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    reports.into_iter().partition(|x| {
        if !report_is_safe(x) {
            return false;
        }
        true
    })
}

// part 2
fn fixed_reports(reports: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut fixed_reports = Vec::new();
    
    for report in reports {
        // for each report, check if it can be fixed by removing one element
        // use iterator filtering to avoid making a copy of the report each time
        let fixed = (0..report.len()).any(|i| {
            let modified_report = report.iter()
                .enumerate()
                // exclude current index
                .filter(|&(index, _)| index != i)
                // collect the values
                .map(|(_, &value)| value)
                .collect();

            // check if the modified report is safe
            report_is_safe(&modified_report)
        });

        // if it can be fixed, clone to copy to the fixed reports
        if fixed {
            fixed_reports.push(report.clone());
        }
    }
    fixed_reports
}


fn main() {
    let input1 = parse_input("input.txt").unwrap();
    // part 1:
    let reports = safe_reports(input1);
    let safe_reports = reports.0.len();
    println!("{}", safe_reports);

    // part 2:
    let fixed_reports = fixed_reports(&reports.1);
    let total_safe_reports = safe_reports + fixed_reports.len();
    println!("{}", total_safe_reports);
}


