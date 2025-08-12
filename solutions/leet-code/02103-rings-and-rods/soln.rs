impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut ans = vec![vec![];10];
        rings.as_bytes().chunks(2).for_each(|x| ans[(x[1]-48) as usize].push(x[0]));
        (0..10).for_each(|x| {ans[x].sort(); ans[x].dedup();});
        ans.into_iter().filter(|x| x.len()==3).count() as i32
    }
}
