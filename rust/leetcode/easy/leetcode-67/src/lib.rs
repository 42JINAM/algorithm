pub struct Solution;

impl Solution {
    // fail ...
    fn ft_atoi(num: &String) -> u128 {
        let mut number: u128 = 0;
        for n in num.chars() {
            number <<= 1;
            if n == '1' {
                number += 1;
            }
        }
        number
    }
    pub fn add_binary(a: String, b: String) -> String {
        let a = Self::ft_atoi(&a);
        let b = Self::ft_atoi(&b);

        format!("{:b}", a + b)
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
