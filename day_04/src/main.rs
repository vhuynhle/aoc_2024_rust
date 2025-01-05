fn main() {
    let grid = read_input();
    println!("Part 1: {}", count_str_in_grid(&grid, "XMAS"));
    println!("Part 2: {}", count_xmas(&grid));
}

fn read_input() -> Vec<String> {
    let grid: Vec<String> = std::io::stdin()
        .lines()
        .map(|result| result.expect("Error reading input"))
        .collect();

    assert!(grid.len() >= 3);
    assert!(grid[0].len() >= 3);

    grid
}

fn count_str_in_grid(grid: &[String], substr: &str) -> i32 {
    let nrows = grid.len();
    let ncols = grid.first().unwrap().len();
    let reverse_substr: String = substr.chars().rev().collect();
    let mut result = 0;

    for line in grid {
        result += count_substr(line, substr);
        result += count_substr(line, &reverse_substr);
    }

    for i in 0..ncols {
        let col = get_col(grid, i);
        result += count_substr(&col, substr);
        result += count_substr(&col, &reverse_substr);
    }

    // Diagonal r + c = d
    for d in 0..(nrows + ncols - 1) {
        let diagonal = get_forward_diagonal(grid, d);
        result += count_substr(&diagonal, substr);
        result += count_substr(&diagonal, &reverse_substr);
    }

    // Diagonal r - c + (n - 1) = d
    for d in 0..(nrows + ncols - 1) {
        let diagonal = get_backward_diagonal(grid, d);
        result += count_substr(&diagonal, substr);
        result += count_substr(&diagonal, &reverse_substr);
    }

    result
}

fn count_substr(mut whole: &str, substr: &str) -> i32 {
    let mut count = 0;
    while let Some(pos) = whole.find(substr) {
        count += 1;
        whole = &whole[(pos + substr.len())..];
    }

    count
}

fn get_col(grid: &[String], col: usize) -> String {
    grid.iter()
        .map(|line| line.as_bytes()[col] as char)
        .collect()
}

/// Diagonal r + c = d
fn get_forward_diagonal(grid: &[String], d: usize) -> String {
    let m = grid.len();
    let n = grid.first().unwrap().len();
    let start_r = if d < m { d } else { m - 1 };
    let end_r = if d < (n - 1) { 0 } else { d - (n - 1) };

    let mut res = String::new();
    let mut r = start_r;
    while r >= end_r {
        let c = d - r;
        res.push(grid[r].as_bytes()[c] as char);

        match r.overflowing_sub(1) {
            (_, true) => break,
            (out, _) => r = out,
        }
    }

    res
}

/// Diagonal d: (r - c) + (n - 1) = d
fn get_backward_diagonal(grid: &[String], d: usize) -> String {
    let m = grid.len();
    let n = grid[0].len();
    let start_c = if d < n { n - 1 - d } else { 0 };
    let end_c = std::cmp::min(n - 1, (m - 1) + (n - 1) - d);

    let mut res = String::new();
    for c in start_c..=end_c {
        let r = d + c - (n - 1);
        res.push(grid[r].as_bytes()[c] as char);
    }

    res
}

fn count_xmas(grid: &[String]) -> i32 {
    let mut count = 0;
    let is_xmas = |s| -> bool { s == "MAS" || s == "SAM" };

    for r in 0..grid.len() - 2 {
        for c in 0..grid[0].len() - 2 {
            let mut diag1 = String::new();
            diag1.push(grid[r].as_bytes()[c] as char);
            diag1.push(grid[r + 1].as_bytes()[c + 1] as char);
            diag1.push(grid[r + 2].as_bytes()[c + 2] as char);

            let mut diag2 = String::new();
            diag2.push(grid[r + 2].as_bytes()[c] as char);
            diag2.push(grid[r + 1].as_bytes()[c + 1] as char);
            diag2.push(grid[r].as_bytes()[c + 2] as char);

            if is_xmas(diag1) && is_xmas(diag2) {
                count += 1;
            }
        }
    }

    count
}
