impl Solution {
    pub fn num_sub(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        s.split("0").map(|x| (((x.len()*(x.len()+1))as i64/2)%MOD) as i32).sum::<i32>()
    }
}
