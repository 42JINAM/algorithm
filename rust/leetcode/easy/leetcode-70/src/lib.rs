pub struct Solution;

impl Solution {
    // n = 1 -> 1
    // n = 2 -> (1, 1), (2)
    // n = 3 -> (1, 1), (2, 1) , (1, 2)
    // n = 4 -> (1, 1, 1, 1), (2, 1, 1), (2, 2), (1, 2, 1)
    // n = 5 -> (1, 1, 1, 1), (1, 1, 1, 2), (1, 2, 2), (2, 1, 1, 1), (2, 2, 1)
    // n = (n - 1) + (n - 2)
    fn fibo(n: i32) -> i32 {
        // if n == 1 {
        //     return 1;
        // } else {
        //     return Self::fibo(n - 1) + Self::fibo(n - 2);
        // }
        if n <= 2 {
            return n;
        }
        let (mut a, mut b) = (1, 2);
        for _ in 3..=n {
            let c = a + b;
            a = b;
            b = c;
        }
        b
    }
    pub fn climb_stairs(n: i32) -> i32 {
        Self::fibo(n)
    }
    // ===== 여기에 LeetCode 터미널 코드 붙여넣기 =====
    // pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> { ... }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // ===== LeetCode Examples 여기에 추가 =====
    // #[test] fn example_1() { assert_eq!(Solution::함수명(args), expected); }
}
