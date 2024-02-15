impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.clone() + &str2 != str2.clone() + &str1 {
            return "".to_string();
        } 
        let size = gcd(str1.len(), str2.len());
        str1[0..size].to_string()
    }
}

fn gcd(a: usize, b: usize) -> usize{
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
