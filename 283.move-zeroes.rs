impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        nums.sort_by_key(|&x| x == 0) 
    }
}
