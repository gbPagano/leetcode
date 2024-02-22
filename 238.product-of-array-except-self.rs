impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1; nums.len()];
        nums.iter().enumerate().fold(1, |acc, (idx, val)| {
            res[idx] = acc;
            acc * val
        });
        nums.iter().enumerate().rev().fold(1, |acc, (idx, val)| {
            res[idx] *= acc;
            acc * val
        });
        res
    }
}
