pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];
        let mut stack = vec![];

        for (i, &temp) in temperatures.iter().enumerate() {
            while let Some(&top) = stack.last() {
                if temperatures[top] >= temp {
                    break;
                }
                result[stack.pop().unwrap()] = (i - top) as i32;
            }
            stack.push(i);
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
