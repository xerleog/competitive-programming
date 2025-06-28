impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut val =  nums.clone().into_iter().enumerate().map(|(a,b)| (b,a)).collect::<Vec<_>>();
        val.sort_by(|a,b| b.0.cmp(&a.0));
        let mut sol :Vec<(i32,usize)> = val.into_iter().take(k as usize).collect();
        sol.sort_by(|a,b| a.1.cmp(&b.1));
        sol.into_iter().map(|x| x.0).collect()
    }
}
