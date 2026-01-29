pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashSet;
    let mut res = 0i32;
    let mut substring = HashSet::new();
    for c in s.chars() {
        if !substring.insert(c) {
            let ln = substring.len() as i32;
            if ln > res {
                res = ln;
            }
            substring = HashSet::new();
            substring.insert(c);
        }
    }
    res.max(substring.len() as i32)
}
