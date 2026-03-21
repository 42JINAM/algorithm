pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut s_map: HashMap<char, char> = HashMap::new();
        let mut t_map: HashMap<char, char> = HashMap::new();

        for (sc, tc) in s.chars().zip(t.chars()) {
            if *s_map.entry(sc).or_insert(tc) != tc {
                return false;
            }
            if *t_map.entry(tc).or_insert(sc) != sc {
                return false;
            }
        }
        true
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
