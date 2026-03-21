use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut res = vec![0, 0];

        //[1,2,2,4]
        //nums[1] => -2
        //nums[1] => +2 / res[0] => 2 duplicated num
        //
        //missing number
        //[-1,-2,2,-4];
        // index 0, 1, 2, 3
        for j in 0..nums.len() {
            let value = nums[j].abs();
            let i = (value - 1) as usize;
            if nums[i] > 0 {
                nums[i] *= -1;
            } else {
                res[0] = value;
            }
        }

        for (i, &num) in nums.iter().enumerate() {
            if num > 0 {
                res[1] = (i as i32) + 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // ===== LeetCode Examples 여기에 추가 =====
    // #[test] fn example_1() { assert_eq!(Solution::함수명(args), expected); }
}
