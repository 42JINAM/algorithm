pub struct Solution;

trait Stack<T> {
    fn push_stack(&mut self, item: T) -> String;
    fn pop_stack(&mut self) -> String;
}

impl<T> Stack<T> for Vec<T> {
    fn push_stack(&mut self, item: T) -> String {
        self.push(item);
        "Push".to_string()
    }
    fn pop_stack(&mut self) -> String {
        self.pop();
        "Pop".to_string()
    }
}

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut commands: Vec<String> = Vec::new();
        let mut stack: Vec<i32> = Vec::new();

        for i in 1..=n {
            if target == stack {
                break;
            }
            commands.push(stack.push_stack(i));
            if (!target.contains(&i)) {
                commands.push(stack.pop_stack());
            }
        }

        commands
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
