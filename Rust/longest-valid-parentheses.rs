/**
Naive recursion version
 */
use std::collections::HashMap;
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let bs = s.as_bytes();
        fn search<'a>(bs: &'a [u8], memo: &mut HashMap<&'a [u8], (usize, usize)>)
            -> (usize, usize) {
            if bs.is_empty() {
                return (0, 0);
            }
            if let Some(res) = memo.get(bs) {
                //ok i know this is bad, but i'm lazy and making this 'dynamic-programming' should not be my work:)
                return res.clone();
            }
            match bs[0] {
                b')' => (0, 0),
                b'(' => if bs.len() > 1 {
                    if bs[1] == b')' {
                        let (len, next) = search(&bs[2..], memo);
                        memo.insert(bs, (len + 2, next + 2));
                        (len + 2, next + 2)
                    } else {
                        let (len, next) = search(&bs[1..], memo);
                        if next + 1 < bs.len() && bs[next + 1] == b')' {
                            let (len_next, next_next) = search(&bs[next + 2..], memo);
                            memo.insert(bs, (len + 2 + len_next, next + 2 + next_next));
                            (len + 2 + len_next, next + 2 + next_next)
                        } else {
                            memo.insert(bs, (0, 0));
                            (0, 0)
                        }
                    }
                } else {
                    memo.insert(bs, (0, 0));
                    (0, 0)
                },
                _ => (0, 0) //evil! no type restriction:(
            }
        }
        let mut ans = 0;
        let mut memo = HashMap::new();
        for i in 0..bs.len() {
            let res = search(&bs[i..], &mut memo).0 as i32;
            ans = std::cmp::max(ans, res);
        }
        ans
    }
}
