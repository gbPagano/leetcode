impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut res = vec![-1, -1]; 
        for i in 0..nums.len()-1 {
            if nums[i] == nums[i+1] {
                let rep = nums[i];
                let miss = (nums.len()*(nums.len()+1) / 2) as i32 - nums.iter().sum::<i32>() + rep;
                return vec![rep, miss];
            }
        }
        vec![]
    }
}
