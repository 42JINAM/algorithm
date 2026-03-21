pub struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        //A.iter().chain(B.iter)
        //After first iterator ends, chain to the second iterator
        //nums.iter() give us a reference, but we need owned values
        //to use cloned(), we can clone each element to collect them into a new vector
        let concatenation = nums.iter().chain(nums.iter()).cloned().collect();

        concatenation

        // another solution from leetcode using cycle.
        // A.iter().cycle() : cycle() is a function for endlessly repeating iterator.
        // nums.iter().cycle().take(nums.len() * 2).cloned().collect()
        // take consumes the iterator and creates a new Take iterator that means we need cloned
        // again.
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
