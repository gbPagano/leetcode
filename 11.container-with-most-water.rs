impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, height.len() - 1);
        let mut area = 0;
        while i < j {
            area = area.max(height[i].min(height[j]) * (j - i) as i32);
            if height[i] > height[j] {
                j -= 1;
            } else {
                i += 1;
            }
        }
        area
    }
}
