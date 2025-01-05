const NUM_DIGITS: usize = 10;
const NUM_LOWERCASES: usize = 26;
const NUM_UPPERCASES: usize = 26;
const ALPHA_NUMERIC_SIZE: usize = NUM_DIGITS + NUM_LOWERCASES + NUM_UPPERCASES;

#[derive(Clone)]
struct Point {
    row: usize,
    col: usize,
}

fn main() {
    let mut grid = vec![vec![]; ALPHA_NUMERIC_SIZE];
    let mut ncols = 0;
    let mut row = 0;
    for line in std::io::stdin()
        .lines()
        .map(|result| result.expect("Error reading input file"))
    {
        let line = line.trim();
        ncols = line.len();
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_alphanumeric() {
                grid[char_index(ch)].push(Point { row, col });
            }
        }

        row += 1;
    }

    let nrows = row;
    part1(&grid, nrows, ncols);
    part2(&grid, nrows, ncols);
}

fn char_index(ch: char) -> usize {
    if ch.is_ascii_digit() {
        ch as usize - '0' as usize
    } else if ch.is_ascii_lowercase() {
        (ch as usize - 'a' as usize) + NUM_DIGITS
    } else {
        (ch as usize - 'A' as usize) + NUM_DIGITS + NUM_LOWERCASES
    }
}

fn part1(grid: &[Vec<Point>], nrows: usize, ncols: usize) {
    let mut antinode_map: Vec<Vec<bool>> = vec![vec![false; ncols]; nrows];
    let mut mark_antinode = |row: usize, col: usize| {
        if (row < nrows) && (col < ncols) {
            antinode_map[row][col] = true;
        }
    };

    for nodes in grid {
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let dx = nodes[i].row.wrapping_sub(nodes[j].row);
                let dy = nodes[i].col.wrapping_sub(nodes[j].col);
                mark_antinode(nodes[i].row.wrapping_add(dx), nodes[i].col.wrapping_add(dy));
                mark_antinode(nodes[j].row.wrapping_sub(dx), nodes[j].col.wrapping_sub(dy));
            }
        }
    }

    println!("Part 1: {}", count_set_entries(&antinode_map));
}

fn part2(grid: &[Vec<Point>], nrows: usize, ncols: usize) {
    let mut antinode_map: Vec<Vec<bool>> = vec![vec![false; ncols]; nrows];

    for nodes in grid {
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let dx = nodes[i].row.wrapping_sub(nodes[j].row);
                let dy = nodes[i].col.wrapping_sub(nodes[j].col);

                let Point { mut row, mut col } = nodes[i];
                while row < nrows && col < ncols {
                    antinode_map[row][col] = true;
                    row = row.wrapping_add(dx);
                    col = col.wrapping_add(dy);
                }
                let Point { mut row, mut col } = nodes[j];
                while row < nrows && col < ncols {
                    antinode_map[row][col] = true;
                    row = row.wrapping_sub(dx);
                    col = col.wrapping_sub(dy);
                }
            }
        }
    }

    println!("Part 2: {}", count_set_entries(&antinode_map));
}

fn count_set_entries(grid: &[Vec<bool>]) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|v| **v).count())
        .sum()
}
