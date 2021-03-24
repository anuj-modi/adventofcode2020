use std::fs;

pub fn solve() {
    let contents =
        fs::read_to_string("input/one.txt").expect("Something went wrong reading the file");

    let mut nums = vec![];

    for line in contents.split("\n") {
        if let Ok(n) = line.parse::<u32>() {
            nums.push(n);
        }
    }

    for x in &nums {
        for y in &nums {
            if x + y == 2020 {
                println!("{}", x * y)
            }

            for z in &nums {
                if x + y + z == 2020 {
                    println!("{}", x * y * z)
                }
            }
        }
    }
}
