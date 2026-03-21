use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // let mut freq: HashMap<i32, i32> = HashMap::new();
        // let mut len = nums.len() as i32 / 2;
        // let mut result = 0;
        //
        // for n in nums {
        //     let count = freq.entry(n).or_insert(0);
        //     *count += 1;
        //     if *count >= len as i32 {
        //         len = *count;
        //         result = n;
        //     }
        // }
        // result
        //
        // Boyer-Moore Voting Algorithm
        let (mut candidate, mut count) = (0, 0);

        for &n in &nums {
            if count == 0 {
                candidate = n;
            }
            if candidate == n {
                count += 1;
            } else {
                count -= 1;
            }
        }
        candidate
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
