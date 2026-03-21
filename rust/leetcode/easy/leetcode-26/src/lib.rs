pub struct Solution;
// remove duplicates in-place
// inplace :
// modify input directly using O(1) extra space, no new arrays
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut len = nums.len();
        let mut k: i32 = 0;

        for i in 0..len {
            // k -> empty space
            // k - 1 -> last num
            if k == 0 || nums[i] != nums[(k - 1) as usize] {
                nums[k as usize] = nums[i];
                k += 1;
            }
        }
        k
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
