impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut ans = vec![nums[0];nums.len()];
        (1..nums.len()).for_each(|x| ans[x]=ans[x-1]+nums[x]);
        ans.into_iter().filter(|&x| x==0).count() as i32
    }
}
