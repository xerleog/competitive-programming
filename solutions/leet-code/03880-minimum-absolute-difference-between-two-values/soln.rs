impl Solution {
    pub fn min_absolute_difference(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;
        for i in 0..nums.len()
        {
            for j in 0..nums.len()
            {
                if nums[i]==1 && nums[j]==2
                { ans = ans.min(((i-j) as i32).abs());}
            }
        }
        if ans==i32::MAX { return -1;} else { ans}
    }
}
