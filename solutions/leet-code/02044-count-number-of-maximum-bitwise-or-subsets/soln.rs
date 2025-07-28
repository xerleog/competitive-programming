use std::collections::BTreeMap;
impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut ans :BTreeMap<i32,i32>= BTreeMap::new();
        let mut n = nums.len();
        for i in 0..(1<<n)
        {
            let mut temp = 0;
            for j in 0..n
            {
                if ((1<<j)&i)!=0
                {   temp|=nums[j];
                }
            }
            *ans.entry(temp).or_default()+=1; 
        } 
        *ans.iter().next_back().unwrap().1
    }
}
