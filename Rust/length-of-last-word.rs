impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim().split(" ").collect::<Vec<&str>>().last().unwrap_or(&"").len() as i32
    }
}
