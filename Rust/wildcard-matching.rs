//Naive memorization solution
use std::collections::HashMap;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let ss = s.as_bytes();
        let ps = p.as_bytes();
        let mut memo = HashMap::new();

        fn search<'a>(
            s: &'a [u8], p: &'a [u8],
            memo: &mut HashMap<(&'a [u8], &'a [u8]), bool>
        ) -> bool {

            if let Some(&b) = memo.get(&(s, p)) {
                return b;
            }
            if p.is_empty() {
                let ans = s.is_empty();
                memo.insert((s, p), ans);
                return ans;
            }
            if s.is_empty() {
                let ans = match p[0] {
                    b'?' => false,
                    b'*' => search(s, &p[1..], memo),
                    c => false
                };
                memo.insert((s, p), ans);
                ans
            }
            else {
                let ans = match p[0] {
                    b'?' => search(&s[1..], &p[1..], memo),
                    b'*' =>
                        search(&s[0..], &p[1..], memo) ||
                        search(&s[1..], &p[0..], memo) ||
                        search(&s[1..], &p[1..], memo),
                    c => c == s[0] && search(&s[1..], &p[1..], memo)
                };
                memo.insert((s, p), ans);
                ans
            }
        }
        search(&ss, &ps, &mut memo)
    }
}

//Real dynamic programming
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let ss = s.as_bytes();
        let ps = p.as_bytes();
        let mut memo = vec![vec![false; ps.len() + 1]; ss.len() + 1];
        for i in (0..ss.len() + 1).rev() {
            for j in (0..ps.len() + 1).rev() {
                memo[i][j] = if j == ps.len() {
                    i == ss.len()
                } else if i == ss.len() {
                    match ps[j] {
                        b'?' => false,
                        b'*' => memo[i][j + 1],
                        c => false
                    }
                } else {
                    match ps[j] {
                        b'?' => memo[i + 1][j + 1],
                        b'*' =>
                            memo[i][j + 1] ||
                            memo[i + 1][j] ||
                            memo[i + 1][j + 1],
                        c => c == ss[i] && memo[i + 1][j + 1]
                    }
                }
            }
        }
        memo[0][0]
    }
}
