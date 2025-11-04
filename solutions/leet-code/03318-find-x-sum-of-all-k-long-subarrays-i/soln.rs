impl Solution {
    pub fn sum_ans(val: Vec<i32>,y:usize) -> i32 {
        let mut ans = vec![0;51];
        val.into_iter().for_each(|x| ans[x as usize]+=1);
        let mut sol = ans.into_iter().enumerate().map(|(a,b)| (b,a)).collect::<Vec<_>>();
        sol.sort();
        return sol[51-y..].into_iter().map(|c| (c.0 * c.1) as i32).sum::<i32>();
    }
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        nums.windows(k as usize).map(|y| Self::sum_ans(y.to_vec(),x as usize)).collect::<Vec<_>>()   
    }
}
