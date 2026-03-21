pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();

        let (n, m) = (haystack.len(), needle.len());

        if m == 0 {
            return 0;
        }

        if m > n {
            return -1;
        }

        let mut k = 0;
        while k <= n - m {
            let mut j = 0;
            while j < m && haystack[k + j] == needle[j] {
                j += 1;
            }
            if j == m {
                return k as i32;
            }
            k += 1;
        }
        -1
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
