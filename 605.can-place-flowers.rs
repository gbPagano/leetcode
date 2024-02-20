impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut can_be_planted = 0;
        let mut idx = 0;
        while idx < flowerbed.len() {
            if flowerbed[idx] == 1 {
                idx += 2;
            } else if (idx == 0 || flowerbed[idx-1] == 0) 
                && (idx == flowerbed.len() - 1 || flowerbed[idx+1] == 0) {
                idx += 2;
                can_be_planted += 1;
            } else {
                idx += 1;
            }
        }
        can_be_planted >= n
    }
}
