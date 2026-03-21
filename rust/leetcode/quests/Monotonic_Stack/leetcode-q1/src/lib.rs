pub struct Solution;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let len = prices.len();
        let mut final_prices = prices.clone();
        let mut stack = Vec::new(); //array of idx 

        //prices = [8,4,6,2,3]
        //prices = [8,9,6,2,3] [8, 9]
        for i in 0..len {
            while let Some(&top) = stack.last() {
                if prices[top] >= prices[i] {
                    final_prices[top] -= prices[i];
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        final_prices
    }
    // ===== 여기에 LeetCode 터미널 코드 붙여넣기 =====
    // pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> { ... }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // ===== LeetCode Examples 여기에 추가 =====
    // #[test] fn example_1() { assert_eq!(Solution::함수명(args), expected); }
    #[test]
    fn case1() {
        assert_eq!(
            Solution::final_prices(vec![8, 4, 6, 2, 6]),
            vec![4, 2, 4, 2, 3]
        );
    }
}
