use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums.dedup();

        let mut res = 0;
        let mut count = 0;
        let mut prev_num = 0;
        for num in nums {
            if num - prev_num == 1 && num - 1 == prev_num {
                count += 1;
            } else {
                res = res.max(count);
                count = 1;
            }
            prev_num = num;
        }
        res.max(count)
    }
}
