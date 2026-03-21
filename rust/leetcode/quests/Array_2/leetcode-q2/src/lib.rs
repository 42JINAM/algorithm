pub struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        //
        // let mut counted = vec![];
        //
        // for &num in &nums {
        //     counted.push(nums.iter().filter(|&&n| n < num).count() as i32);
        // }
        // counted
        //
        //
        //solution by other.
        //
        let (mut bucket, len) = (vec![0; 102], nums.len());
        let mut nums: Vec<i32> = nums.into_iter().collect();

        // nums = [8,1,2,2,3]
        for i in 0..len {
            // to exclude nums[i] itself when counting numbers smaller than nums[i],
            // we increment index by 1
            bucket[nums[i] as usize + 1] += 1;
        }
        // bucket[9] = 1
        // bucket[2] = 1
        // bucket[3] = 1
        // bucket[3] = 2
        // bucket[4] = 1

        for i in 1..102 as usize {
            bucket[i] += bucket[i - 1];
        }
        // bucket[1] = 0
        // bucket[2] = 1
        // bucket[3] = 3
        // bucket[4] = 4
        // bucket[5] = 4
        // bucket[6] = 4
        // bucket[7] = 4
        // bucket[8] = 4
        // bucket[9] = 5

        for i in 0..len {
            nums[i] = bucket[nums[i] as usize];
        }
        // 0 => bucket[8] => 4
        // 1 => bucket[1] => 0
        // 2 => bucket[2] => 1
        // 3 => bucket[2] => 1
        // 4 => bucket[3] => 3
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // ===== LeetCode Examples 여기에 추가 =====
    // #[test] fn example_1() { assert_eq!(Solution::함수명(args), expected); }
}
