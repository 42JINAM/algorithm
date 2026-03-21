pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }
        let mut left = 1;
        let mut right = x / 2;

        while left <= right {
            let mid = left + (right - left) / 2;
            // if mid as i64 * mid as i64 > x as i64 {
            if mid as i64 > x as i64 / mid as i64 {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        right
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
