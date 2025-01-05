fn main() {
    let mut part1_res = 0;
    let mut part2_res = 0;

    for line in std::io::stdin().lines() {
        let line = line.expect("Error reading from stdin");
        let (target, operands) = parse_line(&line);

        if is_valid_equation(target, &operands, false) {
            part1_res += target;
            part2_res += target;
        } else if is_valid_equation(target, &operands, true) {
            part2_res += target;
        }
    }

    println!("Part 1: {}", part1_res);
    println!("Part 2: {}", part2_res);
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let mut iter = line.split_ascii_whitespace();
    let target: u64 = iter
        .next()
        .expect("Target not found")
        .strip_suffix(':')
        .expect("Suffix ':' not found after target")
        .parse()
        .expect("Failed to parse target number");

    let operands: Vec<u64> = iter
        .map(|op_str| op_str.parse().expect("Failed to parse operand"))
        .rev()
        .collect();

    (target, operands)
}

fn is_valid_equation(target: u64, operands: &[u64], extended: bool) -> bool {
    match operands.len() {
        0 => false,
        1 => operands[0] == target,
        _ => {
            if operands[0] <= target
                && is_valid_equation(target - operands[0], &operands[1..], extended)
            {
                true
            } else if operands[0] != 0
                && target % operands[0] == 0
                && is_valid_equation(target / operands[0], &operands[1..], extended)
            {
                return true;
            } else if extended {
                match prefix(target, operands[0]) {
                    Some(new_target) => is_valid_equation(new_target, &operands[1..], true),
                    None => false,
                }
            } else {
                false
            }
        }
    }
}

fn prefix(mut a: u64, mut b: u64) -> Option<u64> {
    if a < b {
        return None;
    }

    a -= b;
    loop {
        if a % 10 != 0 {
            return None;
        }
        a /= 10;
        b /= 10;
        if b == 0 {
            break;
        }
    }

    Some(a)
}
