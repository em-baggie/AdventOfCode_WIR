fn check1a(report: &Vec<i32>) -> bool {
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
            diff = report[i + 1] - report[i]; 
        } else {
            diff = report[i] - report[i + 1]; 
        }

        if diff > 3 || diff < 1 {
            return false;
        }
    }
    true
}

pub fn part_1(reports: Vec<Vec<i32>>) -> usize {
    reports.into_iter()
        .filter(|x| check1a(x)) 
        .count()
}