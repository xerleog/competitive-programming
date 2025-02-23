impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut ans =i32::MAX;
        for i in l..=r
        {
            for j in nums.windows(i as usize)
            {   let temp = j.into_iter().sum::<i32>();
                if temp>0
                {   ans = ans.min(temp);}
            }
        }
        let result = if ans == i32::MAX { -1 } else { ans };
        result
    }
}
