fn read_input() -> (Vec<i32>, Vec<i32>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.expect("Error reading from stdin");
        let mut iter = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("Not an integer"));
        let a = iter.next().expect("Line does not contain 2 integers");
        let b = iter.next().expect("Line does not contain 2 integers");

        v1.push(a);
        v2.push(b);
    }

    (v1, v2)
}

fn difference_score(v1: &mut [i32], v2: &mut [i32]) -> i32 {
    v1.sort();
    v2.sort();

    v1.iter().zip(v2.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn similarity_score(v1: &[i32], v2: &[i32]) -> i32 {
    let mut i = 0;
    let mut j = 0;

    let mut score = 0;
    while i < v1.len() {
        let value = v1[i];

        // Count the occurences of `value` in v1
        let mut occurences_in_v1 = 0;
        while i < v1.len() && v1[i] == value {
            i += 1;
            occurences_in_v1 += 1;
        }

        // Count the occurences of `value` in v2
        let mut occurences_in_v2 = 0;
        while j < v2.len() && v2[j] < value {
            j += 1;
        }
        while j < v2.len() && v2[j] == value {
            j += 1;
            occurences_in_v2 += 1;
        }

        score += value * occurences_in_v1 * occurences_in_v2;
    }

    score
}

fn main() {
    let (mut v1, mut v2) = read_input();

    let d_score = difference_score(&mut v1, &mut v2);
    println!("Part 1: {}", d_score);

    let s_score = similarity_score(&v1, &v2);
    println!("Part 2: {}", s_score);
}
