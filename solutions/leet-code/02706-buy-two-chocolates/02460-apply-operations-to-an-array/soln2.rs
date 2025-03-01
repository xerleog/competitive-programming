impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len()-1).for_each(|x| { if nums[x]==nums[x+1]{ nums[x]*=2; nums[x+1]=0;} });
        nums.sort_by_key(|n| 0.cmp(n));
        nums
        }
}
