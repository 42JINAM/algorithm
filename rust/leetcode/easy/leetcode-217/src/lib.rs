use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map: HashMap<&i32, i32> = HashMap::new();

        for n in &nums {
            let count = map.entry(&n).or_insert(0);
            *count += 1;
            if *count > 1 {
                return true;
            }
        }
        false
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
