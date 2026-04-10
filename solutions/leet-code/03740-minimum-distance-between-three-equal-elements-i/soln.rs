impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;
        if nums.len() < 3 { return -1;}
        for i in 0..nums.len()-2
        {
            for j in i+1..nums.len()-1
            {
                for k in j+1..nums.len()
                {
                    if nums[i] == nums[j] && nums[j] == nums[k] 
                    {
                        let val = ((i-j) as i32).abs()+((j-k) as i32).abs()+((k-i) as i32).abs();
                        ans = ans.min(val);
                    }
                }
            }
        }
        if ans == i32::MAX { return -1; } else { return ans;}
    }
}
