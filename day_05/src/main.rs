use std::{cmp::Ordering, collections::HashSet};

fn main() {
    let rules = read_comparison_rules();
    let comp = |a: &i32, b: &i32| -> Ordering {
        if a == b {
            Ordering::Equal
        } else if rules.contains(&(*b, *a)) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    };
    let is_sorted =
        |seq: &[i32]| -> bool { seq.windows(2).all(|w| comp(&w[0], &w[1]) == Ordering::Less) };

    let mut p1_sum = 0;
    let mut p2_sum = 0;
    for line in std::io::stdin().lines() {
        let line = line.expect("Failed to read input sequence");
        let mut seq = parse_sequence(&line);

        if is_sorted(&seq) {
            p1_sum += seq[seq.len() / 2];
        } else {
            loop {
                seq.sort_by(comp);
                if is_sorted(&seq) {
                    break;
                }
            }
            p2_sum += seq[seq.len() / 2];
        }
    }

    println!("Part 1: {}", p1_sum);
    println!("Part 2: {}", p2_sum);
}

fn read_comparison_rules() -> HashSet<(i32, i32)> {
    let mut line = String::new();
    let mut rules: HashSet<(i32, i32)> = HashSet::new();

    loop {
        line.clear();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Error reading comparision rule");
        let line = line.trim();

        if line.is_empty() {
            break;
        }

        let (a, b) = line.split_once('|').expect("Error parsing comparison rule");
        let a: i32 = a.parse().expect("Error parsing comparison rule");
        let b: i32 = b.parse().expect("Error parsing comparison rule");

        rules.insert((a, b));
    }

    rules
}

fn parse_sequence(line: &str) -> Vec<i32> {
    line.split(',')
        .map(|num_str| num_str.parse().expect("Failed to parse number in sequence"))
        .collect()
}
