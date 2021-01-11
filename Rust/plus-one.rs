impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut i = digits.len() as i32 - 1;

        while i >= 0 && 1 + digits[i as usize] > 9 {
            digits[i as usize] = 0;
            i -= 1;
        }
        if i < 0 && digits[0] == 0 {
            digits.insert(0, 1);
        }
        else {
            digits[i as usize] += 1;
        }
        digits
    }
}
