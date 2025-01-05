use regex::Regex;
use std::sync::LazyLock;

fn main() {
    let lines = read_input();

    let total_sum_mul: i32 = lines.iter().map(|line| sum_mul(line)).sum();
    println!("Part 1: {}", total_sum_mul);

    part2(&lines);
}

fn read_input() -> Vec<String> {
    std::io::stdin()
        .lines()
        .map(|result| result.expect("Error reading input"))
        .collect()
}

fn sum_mul(line: &str) -> i32 {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

    RE.captures_iter(line)
        .map(|c| c.extract())
        .map(|(_, [num1_str, num2_str])| {
            let num1: i32 = num1_str.parse().unwrap_or(0);
            let num2: i32 = num2_str.parse().unwrap_or(0);
            num1 * num2
        })
        .sum()
}

fn part2(lines: &[String]) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;

    for line in lines {
        for capture in re.captures_iter(line) {
            let whole_match = capture.get(0).unwrap().as_str();
            if whole_match == "do()" {
                enabled = true;
            } else if whole_match == "don't()" {
                enabled = false;
            } else if enabled {
                let num1: i32 = capture.get(1).unwrap().as_str().parse().unwrap_or(0);
                let num2: i32 = capture.get(2).unwrap().as_str().parse().unwrap_or(0);
                sum += num1 * num2;
            }
        }
    }

    println!("Part 2: {}", sum);
}
