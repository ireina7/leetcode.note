//Naive memorization solution version 1:
use std::collections::HashMap;
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let xs = s1.as_bytes();
        let ys = s2.as_bytes();
        fn check(
            xs: &[u8], ys: &[u8],
            I_x: usize, J_x: usize,
            I_y: usize, J_y: usize,
            rc: &mut HashMap<(usize, usize, usize, usize), bool>
        ) -> bool {
            if let Some(&b) = rc.get(&(I_x, J_x, I_y, J_y)) {
                return b;
            }
            if I_x == J_x || I_y == J_y {
                return xs[I_x] == ys[I_y];
            }
            let mut ans = false;
            for y in I_y..J_y {
                let x0 = I_x + (y - I_y);
                let x1 = J_x - (y - I_y);
                ans = ans ||
                    (check(xs, ys, I_x, x0, I_y, y, rc) && check(xs, ys, x0+1, J_x, y+1, J_y, rc)) ||
                    (check(xs, ys, x1, J_x, I_y, y, rc) && check(xs, ys, I_x, x1-1, y+1, J_y, rc));
            }
            rc.insert((I_x, J_x, I_y, J_y), ans);
            ans
        }
        let mut rc = HashMap::new();
        check(xs, ys, 0, xs.len()-1, 0, ys.len()-1, &mut rc)
    }
}

//Naive memorization solution version 2(clearer)
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        use std::collections::HashMap;
        let xs = s1.as_bytes();
        let ys = s2.as_bytes();

        fn test<'a>(
            xs: &'a [u8], ys: &'a [u8],
            memo: &mut HashMap<(&'a [u8], &'a [u8]), bool>
        ) -> bool {
            if let Some(&b) = memo.get(&(xs, ys)) {
                return b;
            }
            if xs.len() == 1 && ys.len() == 1 {
                let ans = xs[0] == ys[0];
                memo.insert((xs, ys), ans);
                return ans;
            }
            for i in 1..xs.len() {
                let j = ys.len() - i;
                if  test(&xs[..i], &ys[..i], memo) && test(&xs[i..], &ys[i..], memo) ||
                    test(&xs[..i], &ys[j..], memo) && test(&xs[i..], &ys[..j], memo) {
                    memo.insert((xs, ys), true);
                    return true;
                }
            }
            memo.insert((xs, ys), false);
            false
        }
        let mut memo = HashMap::new();
        test(xs, ys, &mut memo)
    }
}

//TODO: real dynamic programming version
