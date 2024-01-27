use std::collections::HashMap;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut cache: HashMap<(String, String), bool> = HashMap::new();
        Self::is_match_helper(s, p, &mut cache)
    }

    fn is_match_helper(s: String, p: String, cache: &mut HashMap<(String, String), bool>) -> bool {
        if let Some(&result) = cache.get(&(s.clone(), p.clone())) {
            return result;
        }

        if s.len() == 0 && p.len() == 0 {
            cache.insert((s, p), true);
            return true;
        }
        if p.len() == 0 {
            cache.insert((s, p), true);
            return false;
        }

        if s.len() > 0 && (s.chars().nth(0) == p.chars().nth(0) || p.chars().nth(0) == Some('.')) {
            if p.chars().nth(1) == Some('*') {
                let result = Self::is_match_helper(s[1..].to_string(), p[..].to_string(), cache)
                    || Self::is_match_helper(s[..].to_string(), p[2..].to_string(), cache);
                cache.insert((s, p), result);
                return result;
            }
            let result = Self::is_match_helper(s[1..].to_string(), p[1..].to_string(), cache);
            cache.insert((s, p), result);
            return result;
        }

        if p.chars().nth(1) == Some('*') {
            let result = Self::is_match_helper(s[..].to_string(), p[2..].to_string(), cache);
            cache.insert((s, p), result);
            return result;
        }
        cache.insert((s, p), false);
        false
    }
}
