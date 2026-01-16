impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut ans = nums.windows(2).map(|x| x[0]+x[1]).collect::<Vec<_>>();
        ans.sort();
        ans.windows(2).any(|x| x[0]==x[1])
    }
}
