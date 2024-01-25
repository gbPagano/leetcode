impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let num = x.abs().to_string().chars().rev().collect::<String>().parse::<i32>();
        if let Ok(res) = num {
            res * x.signum()
        } else {
            0
        }
    }
}
