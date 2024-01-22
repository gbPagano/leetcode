impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let mut i = 0;
        while i < nums.len() {
            let curr = nums[i];
            if curr > 0 && curr <= nums.len() as i32 && nums[(curr - 1) as usize] != curr {
                let j = (curr - 1) as usize;
                nums.swap(i, j);
                continue;
            }
            i += 1;
        }
        for i in 1..=nums.len() {
            if nums[i - 1] != i as i32 {
                return i as i32;
            }
        }
        (nums.len() + 1) as i32
    }
}
