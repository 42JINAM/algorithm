pub struct Solution;

// we take bits from n right to left using right shift.
// to get exact a right bit, we use & 1 (& operator extracts the last bit).
// To add this bit to the result, we should make room to shift it to left and then use | (or
// operator) to add the bit.

impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut result = 0;
        for i in 0..32 {
            result = (result << 1) | ((n >> i) & 1);
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
