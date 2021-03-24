use std::fs;

fn read() -> std::io::Result<Vec<Vec<char>>> {
    let contents = fs::read_to_string("input/three.txt")?;
    Ok(contents
        .split_whitespace()
        .map(|line| line.chars().collect())
        .collect())
}

fn slope(chars: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    let mut i = 0;
    let mut j = 0;
    let width = chars[0].len();
    let mut trees = 0;

    while i < chars.len() {
        if chars[i][j] == '#' {
            trees += 1;
        }
        j = (j + right) % width;
        i += down;
    }
    trees
}

pub fn solve() {
    let chars = read().unwrap();
    let mut result = 1;
    result *= slope(&chars, 1, 1);
    result *= slope(&chars, 3, 1);
    result *= slope(&chars, 5, 1);
    result *= slope(&chars, 7, 1);
    result *= slope(&chars, 1, 2);

    println!("{}", result);
}
