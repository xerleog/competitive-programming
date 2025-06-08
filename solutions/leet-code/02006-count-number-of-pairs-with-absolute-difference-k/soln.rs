impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len()-1
        {
            for j in i+1..nums.len()
            {
                if (nums[i]-nums[j]).abs() == k
                {   ans+=1;}
            }
        }
        ans
    }
}
