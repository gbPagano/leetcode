impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right: i32 = nums.iter().sum();

        for (idx, n) in nums.iter().enumerate() {
            right -= n;
            if left == right {
                return idx as i32;
            }
            left += n;
        }
        -1
    }
}
