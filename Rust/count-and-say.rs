impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        let prev_res = Self::count_and_say(n - 1).chars().collect::<Vec<char>>();
        let mut res = vec![];

        let mut prev_chr = &prev_res[0];
        let mut count = 1;

        for chr in &prev_res[1..] {
            if chr == prev_chr {
                count += 1;
            } else {
                res.push(format!("{}{}", count, prev_chr));
                count = 1;
            }
            prev_chr = chr;
        }
        res.push(format!("{}{}", count, prev_chr));
        res.join("")
    }
}
