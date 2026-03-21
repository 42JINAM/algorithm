pub struct Solution;

impl Solution {
    fn calculator(x: i32, y: i32, op: &String) -> i32 {
        match op.as_str() {
            "+" => x + y,
            "-" => x - y,
            "*" => x * y,
            "/" => x / y,
            _ => -1,
        }
    }
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        for token in tokens {
            if let Ok(t) = token.parse::<i32>() {
                stack.push(t);
            } else {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(Self::calculator(a, b, &token));
            }
        }
        stack.last().copied().unwrap()
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
