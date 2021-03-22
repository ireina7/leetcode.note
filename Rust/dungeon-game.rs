#[derive(Eq, Debug, PartialEq, Hash, Copy, Clone)]
struct Pos(usize, usize);

impl Solution {
    pub fn calculate_minimum_hp_(dungeon: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        fn search(
            hp: i32, pos: Pos,
            map: &Vec<Vec<i32>>,
            memo: &mut HashMap<Pos, i32>)
        -> i32 {
            if map.is_empty() {
                return hp;
            }
            if let Some(&i) = memo.get(&pos) {
                return hp + i;
            }
            let Pos(i, j) = pos;
            let next_hp = hp + map[i][j];
            if i + 1 == map.len() && j + 1 == map[0].len() {
                memo.insert(pos, map[i][j]);
                return next_hp;
            }
            if j + 1 == map[0].len() {
                let down = search(next_hp, Pos(i + 1, j), map, memo);
                memo.insert(pos, down.min(next_hp) - hp);
                return down.min(next_hp);
            }
            if i + 1 == map.len() {
                let right = search(next_hp, Pos(i, j + 1), map, memo);
                memo.insert(pos, right.min(next_hp) - hp);
                return right.min(next_hp);
            }
            let right = search(next_hp, Pos(i, j + 1), map, memo);
            let down  = search(next_hp, Pos(i + 1, j), map, memo);

            let ans = right.max(down).min(next_hp);
            memo.insert(pos, ans - hp);
            ans
        }
        let mut memo = HashMap::new();
        let res = search(0, Pos(0, 0), &dungeon, &mut memo);
        if res > 0 { 1 } else { 1 - res }
    }

    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        if dungeon.is_empty() || dungeon[0].is_empty() {
            return 0;
        }
        let (m, n) = (dungeon.len(), dungeon[0].len());
        let mut memo = vec![std::i32::MAX; n + 1];
        memo[n-1] = 1;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                memo[j] = memo[j].min(memo[j + 1]) - dungeon[i][j];
                if memo[j] <= 0 {
                    memo[j] = 1;
                }
            }
        }
        memo[0]
    }
}
