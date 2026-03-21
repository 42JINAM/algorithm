pub struct Solution;
//1 001
//2 010
//4 100
//8 1000
//16 10000
//
//0 000
//1 001
//3 011
//7  0111
//15 01111
//
// 1 & 0 -> 0
// 2 & 1 -> 0
// 4 & 3 -> 0
// 8 & 7 -> 0
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // ===== LeetCode Examples 여기에 추가 =====
    // #[test] fn example_1() { assert_eq!(Solution::함수명(args), expected); }
}
