impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len()
        {
            for j in 0..nums.len()
            {
                if i!=j
                {
                    ans = ans.max((nums[i]-1)*(nums[j]-1));
                }
            }
        }
        ans
    }
}
