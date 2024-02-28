use std::collections::VecDeque;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        fn is_vowel(c: char) -> bool {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => true,
                _ => false,
            }
        }
        let k = k as usize;

        let mut queue = VecDeque::new();
        let mut result = 0;
        let mut count = 0;
        for char in s.chars() {
            if queue.len() == k {
                let old = queue.pop_front().unwrap();
                if is_vowel(old) {
                    count -= 1;
                }
            }
            if is_vowel(char) {
                count += 1;
                result = result.max(count);
            }
            queue.push_back(char);
        }
        result
    }
}
