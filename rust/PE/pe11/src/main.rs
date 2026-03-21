use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(20_usize);

    for i in 0..20_usize {
        grid.push(Vec::with_capacity(20_usize));

        grid[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }
    println!("{}", largest_product(&grid));
}

fn largest_product(grid: &Vec<Vec<i32>>) -> i64 {
    let mut largest = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if x + 3 < grid[y].len() {
                // let product = grid[y][x] * grid[y][x + 1] * grid[y][x + 2] * grid[y][x + 3];
                let product: i64 = grid[y][x..x + 4].iter().map(|&v| v as i64).product();
                if product > largest {
                    largest = product;
                }
            }
            if y + 3 < grid.len() {
                let product: i64 = (0..4).map(|dy| grid[y + dy][x] as i64).product();
                if product > largest {
                    largest = product;
                }
            }
            if y + 3 < grid.len() && x + 3 < grid[y].len() {
                let product = (0..4).map(|i| grid[y + i][x + i] as i64).product();
                if product > largest {
                    largest = product;
                }
            }
            if y < grid.len() - 3 && x > 3 {
                let product = grid[y][x] as i64
                    * grid[y + 1][x - 1] as i64
                    * grid[y + 2][x - 2] as i64
                    * grid[y + 3][x - 3] as i64;
                if product > largest {
                    largest = product;
                }
            }
        }
    }
    largest
}
