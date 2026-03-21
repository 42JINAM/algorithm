pub struct Solution;

// Set bit means a bit that is 1 in the number.
// To count the set bit in the given number, I extract last bit and check if it is 1.
// If it's 1, I increment the result by 1.

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut result = 0;
        for i in 0..32 {
            let lastbit = (n >> i) & 1;
            if lastbit == 1 {
                result += 1;
            }
        }
        result
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
