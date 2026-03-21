pub struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut sum = 0;
        for &n in nums.iter() {
            if n == 1 {
                sum += 1;
                max = max.max(sum);
            } else {
                sum = 0;
            }
        }
        max
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
