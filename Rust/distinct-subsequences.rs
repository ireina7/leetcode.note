//Naive memorization recursion
use std::collections::HashMap;
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        fn count<'a>(
            s: &'a[u8], t: &'a[u8],
            memo: &mut HashMap<(&'a[u8], &'a[u8]), i32>)
        -> i32 {
            if let Some(&i) = memo.get(&(s, t)) {
                return i;
            }
            if t.is_empty() {
                memo.insert((s, t), 1);
                return 1;
            }
            if s.is_empty() {
                memo.insert((s, t), 0);
                return 0;
            }
            let ans = if s[0] != t[0] {
                count(&s[1..], t, memo)
            } else if s.len() <= 1 {
                count(&s[1..], &t[1..], memo)
            } else {
                count(&s[1..], &t[1..], memo) + count(&s[1..], &t[0..], memo)
            };
            memo.insert((s, t), ans);
            ans
        }
        let ss = s.as_bytes();
        let ts = t.as_bytes();
        let mut memo = HashMap::new();
        count(&ss, &ts, &mut memo)
    }
}

//Direct naive dynamic programming
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let ss = s.as_bytes();
        let ts = t.as_bytes();
        let sl = ss.len();
        let tl = ts.len();
        let mut memo = vec![vec![0; tl + 1]; sl + 1];
        for i in (0..=sl).rev() {
            for j in (0..=tl).rev() {
                memo[i][j] = if j == tl {
                    1
                } else if i == sl {
                    0
                } else if ss[i] != ts[j] {
                    memo[i+1][j]
                } else if sl - i == 1 {
                    memo[i+1][j+1]
                } else {
                    memo[i+1][j+1] + memo[i+1][j]
                }
            }
        }
        memo[0][0]
    }
}


//Dynamic programming - save memory
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let ss = s.as_bytes();
        let ts = t.as_bytes();
        let sl = ss.len();
        let tl = ts.len();
        let mut memo = vec![0; tl + 1];
        for i in (0..=sl).rev() {
            let mut rc = memo[tl];
            for j in (0..=tl).rev() {
                let tmp = memo[j];
                memo[j] = if j == tl {
                    1
                } else if i == sl {
                    0
                } else if ss[i] != ts[j] {
                    memo[j]
                } else if sl - i == 1 {
                    rc
                } else {
                    rc + memo[j]
                };
                rc = tmp;
            }
        }
        memo[0]
    }
}
