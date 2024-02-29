impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn backtrack(res: &mut Vec<Vec<i32>>, mut nums: Vec<i32>, k: i32, n: i32) {
            if k == 0 && n == 0 {
                res.push(nums.clone());
                return;
            } else if n < 0 {
                return;
            }
            
            let min = nums.iter().max().unwrap() + 1;
            for num in min..=9 {
                if !nums.contains(&num) && num <= n {
                    let mut n_nums = nums.clone();
                    n_nums.push(num);
                    backtrack(res, n_nums, k-1, n-num);
                }
            }
        }

        let mut res: Vec<Vec<i32>> = vec![];
        if k * (k + 1) / 2 > n {
            return res;
        }
        for num in 1..=9 {
            if num <= n {
                backtrack(&mut res, vec![num], k-1, n-num);
            }
        }

        res
    }
}
