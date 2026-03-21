pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut heights = heights.clone();
        let mut largest = 0;
        let mut stack = Vec::new();

        heights.push(0);

        for i in 0..heights.len() {
            while let Some(&top) = stack.last() {
                if heights[top] >= heights[i] {
                    let height = heights[top];
                    let left = if stack.len() > 1 {
                        stack[stack.len() - 2] + 1
                    } else {
                        0
                    };
                    let area = height * ((i - left) as i32);
                    largest = largest.max(area);
                    stack.pop();
                    // we have to stop to extand the area.
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        largest
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
