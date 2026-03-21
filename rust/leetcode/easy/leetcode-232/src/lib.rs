//queue is first in first out
//we have to use stack, but stack is last in first out.
//pop -> the last one out.
//when we pop something from MyQueue, we have to move all to stack2,
//and then move back everythings again.
//

struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack1: Vec::new(),
            stack2: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack1.push(x);
    }

    fn move_items_to_stack2(&mut self) {
        while !self.stack1.is_empty() {
            self.stack2.push(self.stack1.pop().unwrap());
        }
    }

    fn move_items_to_stack1(&mut self) {
        while !self.stack2.is_empty() {
            self.stack1.push(self.stack2.pop().unwrap());
        }
    }

    fn pop(&mut self) -> i32 {
        self.move_items_to_stack2();
        let result = self.stack2.pop().unwrap();
        self.move_items_to_stack1();
        result
    }

    fn peek(&mut self) -> i32 {
        self.move_items_to_stack2();
        let result = *self.stack2.last().unwrap();
        self.move_items_to_stack1();
        result
    }

    fn empty(&self) -> bool {
        self.stack1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // ===== LeetCode Examples 여기에 추가 =====
    // #[test] fn example_1() { assert_eq!(Solution::함수명(args), expected); }
}
