impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        if a.len() < b.len() {
            std::mem::swap(&mut a, &mut b);
        }
        let mut s = String::new();
        let mut sum = 0;
        let mut chars = b.chars().rev();
        for c in a.chars().rev() {
            if c == '1' { sum += 1; }
            if let Some('1') = chars.next() { sum += 1; }
            s.insert(0, if sum & 1 == 1 { '1' } else { '0' });
            sum >>= 1;
        }
        if sum != 0 {
            s.insert(0, '1');
        }
        s
    }
}
