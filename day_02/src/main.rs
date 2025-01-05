use std::cmp::Ordering;

fn main() {
    let reports = read_input();

    let safe_count = reports.iter().filter(|report| is_safe(report)).count();
    println!("Part 1: {}", safe_count);

    let safe_or_almost_safe = reports
        .iter()
        .filter(|report| is_safe(report) || is_almost_safe(report))
        .count();
    println!("Part 2: {}", safe_or_almost_safe);
}

fn read_input() -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];

    for line in std::io::stdin().lines() {
        let line = line.expect("Error reading from stdin");
        let row: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().expect("Not a valid number"))
            .collect();

        result.push(row);
    }

    result
}

fn is_safe(report: &[i32]) -> bool {
    const SAFE_THRESHOLD: i32 = 3;

    if report.len() <= 1 {
        return true;
    }

    match report[0].cmp(&report[1]) {
        Ordering::Equal => false,
        Ordering::Less => report
            .windows(2)
            .all(|w| (w[0] < w[1]) && (w[1] - w[0] <= SAFE_THRESHOLD)),
        Ordering::Greater => report
            .windows(2)
            .all(|w| (w[0] > w[1]) && (w[0] - w[1] <= SAFE_THRESHOLD)),
    }
}

fn is_almost_safe(report: &[i32]) -> bool {
    (0..report.len()).any(|index| {
        let mut sub_report = report.to_vec();
        sub_report.remove(index);
        is_safe(&sub_report)
    })
}
