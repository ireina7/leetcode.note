impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (l1, l2, l3) = (s1.len(), s2.len(), s3.len());
        if l1 + l2 != l3 {
            return false;
        }
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let mut dp: Vec<Vec<bool>> = vec![vec![false; l2+1]; l1+1];
        dp[0][0] = true;

        for i in 1..=l1 {
            if s1[i-1] == s3[i-1] {
                dp[i][0] = dp[i-1][0];
            }
        }
        for j in 1..=l2 {
            if s2[j-1] == s3[j-1] {
                dp[0][j] = dp[0][j-1];
            }
        }

        for i in 1..=l1 {
            for j in 1..=l2 {
                dp[i][j] =
                    (dp[i-1][j] && s3[i+j-1] == s1[i-1]) ||
                    (dp[i][j-1] && s3[i+j-1] == s2[j-1]);
            }
        }
        dp[l1][l2]
    }
}
/*
//This is bad!! Trying to destruct an interleaving string into two sub-part is not a good idea.
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        use std::collections::HashMap;
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let xs = s1.as_bytes();
        let ys = s2.as_bytes();
        let zs = s3.as_bytes();
        use std::cmp;
        fn compare(a: &[u8], b: &[u8]) -> cmp::Ordering {
            a.iter()
                .zip(b)
                .map(|(x, y)| x.cmp(y))
                .find(|&ord| ord != cmp::Ordering::Equal)
                .unwrap_or(a.len().cmp(&b.len()))
        }
        fn search<'a>(
            xs: &'a [u8], ys: &'a [u8], zs: &'a [u8],
            memo: &mut HashMap<(&'a [u8], &'a [u8], &'a [u8]), bool>
        ) -> bool {
            //println!("search {:?}, {:?}, {:?}", xs, ys, zs);
            if let Some(&i) = memo.get(&(xs, ys, zs)) {
                return i;
            }
            if (compare(xs, &zs[..xs.len()]) == cmp::Ordering::Equal &&
                compare(ys, &zs[xs.len()..]) == cmp::Ordering::Equal) {
                    memo.insert((xs, ys, zs), true);
                    return true;
            }
            for i in 1..xs.len() {
                for j in 1..ys.len() + 1 {
                    let k = i + j;
                    let res =
                        search(&xs[..i], &ys[..j], &zs[..k], memo) &&
                        search(&xs[i..], &ys[j..], &zs[k..], memo);
                    if res {
                        memo.insert((xs, ys, zs), true);
                        return true;
                    }
                }
            }
            memo.insert((xs, ys, zs), false);
            false
        }
        let mut memo = HashMap::new();
        search(&xs[..], &ys[..], &zs[..], &mut memo) || search(&ys[..], &xs[..], &zs[..], &mut memo)
    }
}
*/
