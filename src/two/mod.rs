use std::fs;

#[derive(Debug)]
struct Line {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

impl Line {
    fn new(line: &str) -> Self {
        let pieces: Vec<&str> = line.split(&['-', ' ', ':'][..]).collect();
        Self {
            min: pieces[0].parse::<usize>().unwrap(),
            max: pieces[1].parse::<usize>().unwrap(),
            character: String::from(pieces[2]).chars().next().unwrap(),
            password: String::from(pieces[4]),
        }
    }

    fn eval_counts(&self) -> bool {
        let mut count = 0;
        for c in self.password.chars() {
            if c == self.character {
                count += 1
            }
        }
        self.min <= count && count <= self.max
    }

    fn eval_positions(&self) -> bool {
        let password: Vec<char> = self.password.chars().collect();
        let len = password.len();
        len >= self.max
            && ((self.character == password[self.min - 1])
                ^ (self.character == password[self.max - 1]))
    }
}

pub fn solve() {
    let contents =
        fs::read_to_string("input/two.txt").expect("Something went wrong reading the file");

    let mut counts = 0;
    let mut positions = 0;
    for line in contents.split_terminator("\n") {
        let line = Line::new(line);
        if line.eval_counts() {
            counts += 1;
        }

        if line.eval_positions() {
            positions += 1;
        }
    }
    println!("{}, {}", counts, positions);
}
