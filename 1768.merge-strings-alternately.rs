impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let max_size = word1.len().max(word2.len());
        
        let mut result = String::new();
        for idx in 0..max_size {
            if let Some(ch) = word1.chars().nth(idx) {
                result.push(ch);
            }
            if let Some(ch) = word2.chars().nth(idx) {
                result.push(ch);
            }
        }
        result
    }
}
