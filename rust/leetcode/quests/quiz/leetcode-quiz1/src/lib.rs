pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut results = digits.clone();
        let mut i = results.len() as i32 - 1;

        while i >= 0 {
            if digits[i as usize] < 9 {
                results[i as usize] += 1;
                return results;
            }
            results[i as usize] = 0;
            i -= 1;
        }

        results.insert(0, 1);

        results
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
