use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..t {
        let n = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        println!("{}", find_pyta_triplet(n));
    }
}

fn find_pyta_triplet(n: i32) -> i64 {
    let mut max: i64 = -1;
    'outer: for a in (3..n).rev() {
        for b in (a + 1)..n {
            let c = n - (a + b);
            if c <= b {
                break;
            }
            if a.pow(2) + b.pow(2) == c.pow(2) {
                max = a as i64 * b as i64 * c as i64;
                break 'outer;
            }
        }
    }
    return max;
}
