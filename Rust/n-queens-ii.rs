impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        fn safe(i: usize, j: usize, qs: &Vec<usize>) -> bool {
            let xy_abs = |x: usize, y: usize| (x as i32 - y as i32).abs();
            qs.iter().enumerate().all(|(x, &y)| {
                y != j && xy_abs(x, i) != xy_abs(y, j)
            })
        }
        fn search (
            i: usize, n: usize,
            qs: Vec<usize>,
            map: &mut Vec<Vec<usize>>
        ) -> &Vec<Vec<usize>> {
            if i >= n {
                map.push(qs);
                return map;
            }
            let mut xs = qs.clone();
            for j in 0..n {
                if !safe(i, j, &qs) {
                    continue;
                }
                xs.push(j);
                search(i + 1, n, xs.clone(), map);
                xs.pop();
            }
            map
        }
        let mut ans = vec![];
        search(0, n as usize, vec![], &mut ans).len() as i32
    }
}
