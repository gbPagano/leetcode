impl Solution {
    pub fn is_valid(s: String) -> bool {
        // (){}[]
        if s.len() % 2 == 1 {
            return false;
        }
        
        let mut open: Vec<char> = Vec::new();
        for char in s.chars() {
            match char {
                '(' => open.push(')'),
                '{' => open.push('}'),
                '[' => open.push(']'),
                _ => {
                    if open.pop() != Some(char) {
                        return false
                    } 
                },
            }
        }
        open.is_empty()
    }
}
