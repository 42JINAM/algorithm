pub struct Solution;

impl Solution {
    fn parsing_log(log: &String) -> (usize, String, i32) {
        let token: Vec<&str> = log.split(':').collect();
        (
            token[0].parse().unwrap(),
            token[1].to_string(),
            token[2].parse().unwrap(),
        )
    }
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut times = vec![0; n as usize];
        let mut stack: Vec<usize> = Vec::new();
        let mut prev_time = 0;

        for log in logs {
            let (id, state, time) = Self::parsing_log(&log);
            if state == "start" {
                if !stack.is_empty() {
                    times[*stack.last().unwrap()] += (time - prev_time) as i32;
                }
                stack.push(id);
                prev_time = time;
            } else {
                times[id] += time - prev_time + 1;
                stack.pop().unwrap();
                prev_time = time + 1;
            }

            // prevs = current;
        }

        times
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
