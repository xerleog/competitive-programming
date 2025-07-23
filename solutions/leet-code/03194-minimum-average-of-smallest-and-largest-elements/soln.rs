impl Solution {
    pub fn minimum_average(nums: Vec<i32>) -> f64 {
        let mut ans = nums.clone();
        let mut sol = f64::MAX;
        ans.sort();
        while ans.len()>=2
        {
            let l = ans.remove(0);
            let r = ans.pop().unwrap();
            let m = (l+r)as f64/2.0;
            sol = sol.min(m);
        }
        sol

    }
}
