pub struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut start = 0;

        for i in 1..=nums.len() {
            if i == nums.len() || nums[i] != nums[i - 1] + 1 {
                if i - 1 == start {
                    result.push(nums[start].to_string());
                } else {
                    result.push(format!("{}->{}", nums[start], nums[i - 1]));
                }
                start = i;
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
