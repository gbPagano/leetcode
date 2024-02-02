impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut major = nums[0];

        for &item in nums.iter().skip(1) {
            if item == major {
                count += 1
            } else if count == 0 {
                major = item;
                count += 1;
            } else {
                count -= 1;
            }
        }
        major
    }
}
