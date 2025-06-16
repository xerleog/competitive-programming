impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        for i in 0..nums.len()-1
        {
            for j in i+1..nums.len()
            {
                ans = ans.max(nums[j]-nums[i]);
            }
        }
        if ans <1
        {   -1 }
        else
        {ans }
    }
}
