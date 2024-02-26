struct Keyboard {}
impl Keyboard {
    fn from_char(c: char) -> Vec<char> {
        match c {
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['d', 'e', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
            _ => vec![],
        }
    }
}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let initial_letters: Vec<String> = Keyboard::from_char(digits.chars().nth(0).unwrap_or('0'))
            .iter()
            .map(|c| c.to_string())
            .collect();
        if initial_letters.is_empty() {
            return result;
        }
        Self::backtrack(&mut result, digits.chars().skip(1).collect::<String>(), &initial_letters);
        result
    }

    fn backtrack(result: &mut Vec<String>, digits: String, letters: &Vec<String>) {
        let next_letters: Vec<String> = Keyboard::from_char(digits.chars().nth(0).unwrap_or('0'))
            .iter()
            .map(|c| c.to_string())
            .collect();
        if next_letters.is_empty() {
            *result = letters.clone();
        } else {
            Self::backtrack(result, digits.chars().skip(1).collect::<String>(), &next_letters);
            let mut new_res = vec![];
            for letter in letters {
                for suffix in result.iter() {
                    new_res.push(letter.clone() + &suffix);
                }
            }
            *result = new_res;
        }
    }
}
