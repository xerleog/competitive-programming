use std::collections::BTreeMap;
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut ans :BTreeMap<i32,i32>= BTreeMap::new();
        let mut sum = 0;
        nums.into_iter().for_each(|x| *ans.entry(x).or_default()+=1);
        let sol = ans.into_iter().collect::<Vec<_>>();
        for a in sol.windows(2)
        {
            if a[1].0-a[0].0==1
            {   sum = sum.max(a[0].1 + a[1].1);}
        }
        sum
    }
}
