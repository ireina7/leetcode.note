use std::collections::{
    HashMap, HashSet
};
impl Solution {
    fn word_break_impl(
        s: String,
        word_set: &HashSet<String>,
        memo: &mut HashMap<String, bool>)
    -> bool {
        if let Some(&b) = memo.get(&s) {
            return b;
        }
        if word_set.contains(&s) {
            memo.insert(s, true);
            return true;
        }
        for i in 1..s.len() {
            let left = &s[0..i];
            let right = &s[i..];
            if word_set.contains(right) {
                if Self::word_break_impl(left.to_string(), word_set, memo) {
                    memo.insert(s, true);
                    return true;
                }
            }
        }
        memo.insert(s, false);
        false
    }

    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word_set: HashSet<String> = word_dict.iter().cloned().collect();
        let mut memo = HashMap::new();
        Self::word_break_impl(s, &word_set, &mut memo)
    }
}
