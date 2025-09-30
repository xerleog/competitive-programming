impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let mut ini = nums.clone();
        let mut fini = vec![];
        while ini.len()>=2
        {
            for i in ini.windows(2).map(|a| a[0]+a[1])
            {
                fini.push(i%10);
            }
            ini.clear();
            ini=fini.clone();
            fini.clear();
        }
        ini[0]
    }
}
