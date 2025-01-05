fn main() {
    let mut grid = read_input();
    let (r, c) = find_guard(&grid);

    // Part 1
    let records = trace(&grid, r, c).records;
    let record_count: usize = records
        .iter()
        .map(|row| row.iter().filter(|x| **x != 0).count())
        .sum();
    println!("Part 1: {}", record_count);

    // Part 2, brute-force solution
    let mut res_2: usize = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            // only put obstacle on an empty block on the original path, otherwise there's no change
            // in the trace
            if (grid[i][j] == b'.') && (records[i][j] != 0) {
                grid[i][j] = b'#'; // put the obstacle there
                if trace(&grid, r, c).has_loop {
                    res_2 += 1;
                }
                // Backtracking
                grid[i][j] = b'.';
            }
        }
    }
    println!("Part 2: {}", res_2);
}

fn read_input() -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in std::io::stdin()
        .lines()
        .map(|result| result.expect("Error reading input"))
    {
        grid.push(line.trim().as_bytes().to_vec());
    }

    grid
}

fn find_guard(grid: &[Vec<u8>]) -> (usize, usize) {
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, b) in row.iter().enumerate() {
            if b"^>v<".contains(b) {
                return (row_index, col_index);
            }
        }
    }

    panic!("Guard not found");
}

struct TraceResult {
    has_loop: bool,
    records: Vec<Vec<u8>>,
}

fn trace(grid: &[Vec<u8>], mut row: usize, mut col: usize) -> TraceResult {
    const MAX_DIRECTION_CHANGES: u8 = 4;
    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut direction = grid[row][col];
    let mut records = vec![vec![0; ncols]; nrows];
    loop {
        if been_there(&records, row, col, direction) {
            return TraceResult {
                has_loop: true,
                records,
            };
        }
        records[row][col] |= encode_direction(direction);

        for _ in 0..MAX_DIRECTION_CHANGES {
            match next(row, col, direction, nrows, ncols) {
                None => {
                    return TraceResult {
                        has_loop: false,
                        records,
                    }
                }
                Some((next_row, next_col)) => {
                    if grid[next_row][next_col] == b'#' {
                        // Blocked by an obstacle
                        direction = change_direction(direction);
                    } else {
                        row = next_row;
                        col = next_col;
                        break;
                    }
                }
            }
        }
    }
}

fn been_there(records: &[Vec<u8>], row: usize, col: usize, direction: u8) -> bool {
    let d = encode_direction(direction);
    (records[row][col] & d) == d
}

fn encode_direction(direction: u8) -> u8 {
    match direction {
        b'^' => 1 << 0,
        b'>' => 1 << 1,
        b'v' => 1 << 2,
        _ => 1 << 3,
    }
}

fn change_direction(direction: u8) -> u8 {
    match direction {
        b'^' => b'>',
        b'>' => b'v',
        b'v' => b'<',
        _ => b'^',
    }
}

fn next(
    row: usize,
    col: usize,
    direction: u8,
    nrows: usize,
    ncols: usize,
) -> Option<(usize, usize)> {
    match direction {
        b'^' => {
            if row > 0 {
                Some((row - 1, col))
            } else {
                None
            }
        }
        b'>' => {
            if col + 1 < ncols {
                Some((row, col + 1))
            } else {
                None
            }
        }
        b'v' => {
            if row + 1 < nrows {
                Some((row + 1, col))
            } else {
                None
            }
        }
        _ => {
            if col > 0 {
                Some((row, col - 1))
            } else {
                None
            }
        }
    }
}
