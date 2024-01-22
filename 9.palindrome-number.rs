impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let a: String = x.to_string();
        let b: String = a.chars().rev().collect();
        a == b
    }
}
