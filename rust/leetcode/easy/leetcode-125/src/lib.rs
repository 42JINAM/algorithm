pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s = s.to_string();
        s.make_ascii_lowercase();
        s.retain(|c| c.is_ascii_alphanumeric());

        s.as_bytes()
            .iter()
            .rev()
            .zip(s.as_bytes())
            .all(|(a, b)| a == b)
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
