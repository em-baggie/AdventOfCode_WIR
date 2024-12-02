// to be safe:
// - ascending or descending
// - adjacent values to not differ < 1 or > 3
// - removing 1 number can fix

// steps
// isolate and count which ones are safe without needing fixing and which are not safe and need fixing
// work out which ones can be fixed and add these to the count
// return the count

fn safe_reports(reports: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    reports.into_iter().partition(|x| {
        if !x.is_sorted() {
            return false;
        }
        for i in 0..(x.len() - 1) {
            let diff = (x[i] - x[i + 1]).abs();
            if diff > 3 || diff < 1 {
                return false;
            }
        }
        true
    })
}

fn fixed_reports(mut reports: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut fixed_reports = Vec::new();
    for report in reports {
        for n in 0..reports.len() {
            let filtered_reports: Vec<_> = reports.iter()
                .enumerate()
                .filter(|&(i, _)| i != n)
                .map(|(_, v)| v.clone())
                .filter(|x| {
                    if !x.is_sorted() {
                        return false;
                    }
                    for i in 0..(x.len() - 1) {
                        let diff = (x[i] - x[i + 1]).abs();
                        if diff > 3 || diff < 1 {
                            return false;
                        }
                    }
                    true
                })
                .collect();

            fixed_reports.extend(filtered_reports);
        }
    }
    fixed_reports
}

pub fn part_2(reports: Vec<Vec<i32>>) -> usize {
    let safe_without_fixing = safe_reports(reports).0.len();
    let safe_with_fixing = fixed_reports(reports).len();
    safe_without_fixing + safe_with_fixing
}


