pub struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        //counting (mine)
        let mut nums = nums;
        let mut disappeared = vec![];

        for i in 0..nums.len() {
            let index = nums[i].abs() as usize - 1;
            if nums[index] > 0 {
                nums[index] *= -1;
            }
        }

        for i in 0..nums.len() {
            if nums[i] > 0 {
                disappeared.push((i + 1) as i32);
            }
        }

        disappeared
    }
    //other solution (bitset)
    pub fn find_disappeared_numbers_bit(mut nums: Vec<i32>) -> Vec<i32> {
        //nums = [4,3,2,7,8,2,3,1]
        //n -> 8, 8 - 1 = 7,
        //7 | 63 = 63, -> 63 + 1 -> 64 / 64 = 1 (/64 division is implicit in the formula)
        let mut bitset = vec![0u64; ((nums.len() - 1) | 63) + 1];

        // translating num to bit position
        // set bit at each number's position
        for num in &nums {
            bitset[(num / 64) as usize] |= 1 << num % 64;
        }
        let length = nums.len();
        nums.clear();

        //read bits and filter missing numbers
        for i in 1..=length {
            if bitset[i / 64] & 1 << i % 64 == 0 {
                nums.push(i as i32);
            }
        }
        nums
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
