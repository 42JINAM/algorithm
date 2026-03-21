fn main() {
    // find_palindrome();
}
//
// 987 * 789 ==> x
//
// x / 987 = 789
//

fn find_palindrome(n: i64) -> i64 {
    let mut x = 987;
    let mut max = 1;
    while x >= 100 {
        let y = rev(x);
        let num = x * 1000 + y;
        if num < n && check_number(num) {
            max = num;
            break;
        }
        x -= 1;
    }
    return max;
}

fn check_number(num: i64) -> bool {
    let mut check = false;

    for i in (100..=999).rev() {
        let (q, r) = (num / i, num % i);
        if r == 0 && q <= 999 {
            check = true;
            break;
        }
        if q > 999 {
            break;
        }
    }
    return check;
}

fn rev(n: i64) -> i64 {
    let mut num = n;
    let mut reversed = 0;
    while num > 0 {
        reversed = (reversed * 10) + num % 10;
        num /= 10;
    }
    reversed
}
