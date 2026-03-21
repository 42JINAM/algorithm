pub struct Solution;

impl Solution {
    fn ft_itoa(mut n: i32) -> String {
        let s: Vec<char> = ('A'..='Z').collect();
        let mut result: String = String::new();
        let mut negative = false;

        if n < 0 {
            negative = true;
            n = n.abs();
        }

        while n > 0 {
            result.push(s[(n - 1) as usize % 26]);
            n = (n - 1) / 26;
        }
        if negative {
            result.push('-');
        }
        result
    }
    pub fn convert_to_title(column_number: i32) -> String {
        Self::ft_itoa(column_number).chars().rev().collect()
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
