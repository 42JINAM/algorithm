pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        // let s = s.as_bytes();
        // let mut i = s.len() as i32 - 1;
        //
        // while i >= 0 && s[i as usize] == b' ' {
        //     i -= 1;
        // }
        // let mut len = 0;
        // while i >= 0 && s[i as usize] != b' ' {
        //     len += 1;
        //     i -= 1;
        // }
        // len

        //"Hello World   "
        s.chars()
            .rev() //"   dlrow olloW"
            .skip_while(|c| c.is_whitespace()) //"   " -> dlrow olloW //lazy
            .take_while(|c| !c.is_whitespace()) // "dlrow" -> "olloW" //lazy
            .count() as i32
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
