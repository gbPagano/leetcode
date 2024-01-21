use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        
        for (i, n) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - n)) {
                return vec![i as i32, j];
            }
            map.insert(*n, i as i32);
        }
        vec![]
    }   
}
