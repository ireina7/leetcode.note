impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        fn gen(spiral: &mut Vec<Vec<i32>>, x: usize, y: usize, n: usize, num: i32) {
            let mut num = num;
            if n == 1 {
                spiral[x][y] = num;
                return;
            }
            if n == 2 {
                spiral[x  ][y  ] = num;
                spiral[x  ][y+1] = num + 1;
                spiral[x+1][y+1] = num + 2;
                spiral[x+1][y  ] = num + 3;
                return;
            }
            for i in 0..n {
                spiral[x][y + i] = num;
                num += 1;
            }
            for j in 1..(n-1) {
                spiral[x + j][y + n - 1] = num;
                num += 1;
            }
            for i in (0..n).rev() {
                spiral[x + n - 1][y + i] = num;
                num += 1;
            }
            for j in (1..n-1).rev() {
                spiral[x + j][y] = num;
                num += 1;
            }
            gen(spiral, x + 1, y + 1, n - 2, num);
        }
        let mut m = vec![vec![0; n as usize]; n as usize];
        gen(&mut m, 0, 0, n as usize, 1);
        m
    }
}
