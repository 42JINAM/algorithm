pub struct Solution;

//When we replace a number with the sum of the squares of its digits,
//unhappy numbers form a CYCLE instead of reaching 1.
//This creates an ENDLESS LOOP.
//
//To detect this cycle, we use the two-pointer algorithm.
//Imagine a circular running track with two athletes,
//one faster than the other, and eventually the faster one will meet the slower one.
//If the slow's is the same as fast's and it's not 1, the cycle exists in there!

impl Solution {
    pub fn get_sum(n: i32) -> i32 {
        let mut sum = 0;
        let mut x = n;

        while x > 0 {
            let d = x % 10;
            sum += d * d;
            x /= 10;
        }
        sum
    }

    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = n;
        loop {
            slow = Self::get_sum(slow);
            fast = Self::get_sum(Self::get_sum(fast));

            if slow == 1 || fast == 1 {
                return true;
            }
            if slow == fast {
                return false;
            }
        }
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
