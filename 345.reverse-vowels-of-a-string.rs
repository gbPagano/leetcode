impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let (mut left, mut right) = (0, s.len() - 1);
        let mut chars: Vec<char> = s.chars().collect();
        while right > left {
            if !chars[left].is_vowel() {
                left += 1;
                continue;
            }
            if !chars[right].is_vowel() {
                right -= 1;
                continue;
            }
            chars.swap(left, right);
            left += 1;
            right -= 1;
        }
        chars.into_iter().collect()
    }
}


trait IsVowel {
    fn is_vowel(&self) -> bool;
}

impl IsVowel for char {
    fn is_vowel(&self) -> bool {
        matches!(self.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
    }
}
