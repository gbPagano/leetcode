use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, i32> = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            let diff = target - n;
            match hm.get(&diff) {
                Some(&j) => return vec![i as i32, j],
                None => hm.insert(*n, i as i32),
            };
        }
        vec![]
    }
}
