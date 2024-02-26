impl Solution {
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let (mut i, mut j, mut operations) = (0, nums.len() - 1, 0);
        while i < j {
            let curr = nums[i] + nums[j];
            if curr == k {
                operations += 1;
                i += 1;
                j -= 1;
            } else if curr < k {
                i += 1;
            } else {
                j -= 1;
            }
        }
        operations
    }
}
