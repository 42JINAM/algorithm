pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        //nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
        let mut p1 = m - 1;
        let mut p2 = n - 1;
        let mut i = m + n - 1;
        while p1 >= 0 || p2 >= 0 {
            if p2 >= 0 && (p1 < 0 || nums1[p1 as usize] <= nums2[p2 as usize]) {
                nums1[i as usize] = nums2[p2 as usize];
                p2 -= 1;
            } else if p1 >= 0 {
                nums1[i as usize] = nums1[p1 as usize];
                p1 -= 1;
            }
            i -= 1;
        }
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
