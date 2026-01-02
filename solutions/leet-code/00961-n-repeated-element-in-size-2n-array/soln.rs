use std::collections::BTreeMap;
impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut ans :BTreeMap<i32,i32>= BTreeMap::new();
        for i in nums
        {  *ans.entry(i).or_default()+=1;}
        for (i,j) in ans
        {
            if j==n/2
            {
                return i;
            }
        }
        0
    }
}
