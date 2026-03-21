pub struct Solution;

//
// I             1
// V             5
// X             10
// L             50
// C             10
// D             500
// M             1000
//
// II 2
// XXVII -> XX + V + II -> 10 10 5 2 -> 27
//
// I -> IV -> V - I
// I -> IX -> X - I
//
//parsing?
//
//
impl Solution {
    fn roman_value(c: char) -> Option<i32> {
        match c {
            'I' => Some(1),
            'V' => Some(5),
            'X' => Some(10),
            'L' => Some(50),
            'C' => Some(100),
            'D' => Some(500),
            'M' => Some(1000),
            _ => None,
        }
    }

    pub fn roman_to_int(s: String) -> i32 {
        let mut num = 0i32;
        let mut prev = 0;

        for ch in s.chars().rev() {
            if let Some(n) = Self::roman_value(ch) {
                if n < prev {
                    num -= n;
                } else {
                    num += n;
                }
                prev = n;
            }
        }
        num
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
