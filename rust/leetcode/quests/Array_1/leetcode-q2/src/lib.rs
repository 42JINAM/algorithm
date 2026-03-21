pub struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let (first, second) = nums.as_slice().split_at(n as usize);
        let mut result = Vec::new();

        //iter().zip() : it pairs up elements from both iters(itself and the argument of zip) into tuples
        //until end of shorter one.
        for (&x, &y) in first.iter().zip(second.iter()) {
            result.push(x);
            result.push(y);
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
