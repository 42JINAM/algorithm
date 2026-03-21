pub struct Solution;

impl Solution {
    fn cmp_str(str1: &str, str2: &str) -> String {
        let s1 = str1.as_bytes();
        let s2 = str2.as_bytes();
        let mut i = 0;

        while i < s1.len().min(s2.len()) {
            if s1[i] != s2[i] {
                break;
            }
            i += 1;
        }
        str1[..i].to_string()
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut iter = strs.iter();
        let Some(first) = iter.next() else {
            return String::new();
        };

        let mut result = first.clone();

        for s in iter {
            result = Self::cmp_str(result.as_str(), s.as_str());
        }
        result.to_string()
    }
    // ===== 여기에 leetcode 터미널 코드 붙여넣기 =====
    // pub fn get_concatenation(nums: vec<i32>) -> vec<i32> { ... }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // ===== LeetCode Examples 여기에 추가 =====
    // #[test] fn example_1() { assert_eq!(Solution::함수명(args), expected); }
}
