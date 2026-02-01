impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
       let mut ans :i32 = i32::MAX;
       for i in 1..nums.len()-1
       {
            for j in i+1..nums.len()
            {
                ans = ans.min(nums[0]+nums[i]+nums[j]);
                
            }
       }
       ans 
    }
}
