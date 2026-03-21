use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut stdin_iter = stdin.lock().lines();
    let n = stdin_iter
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut s: String = "".to_string();
    for _ in 0..n {
        s.push_str(stdin_iter.next().unwrap().unwrap().as_str());
        s.push('\n');
    }

    println!("{}", sum(s.as_str()));
}

fn sum(input: &str) -> String {
    let lines = input.lines().map(|line| &line[0..15]);
    let nums = lines.filter_map(|s| s.parse::<u64>().ok());

    nums.sum::<u64>().to_string()[0..10].to_string()
}
