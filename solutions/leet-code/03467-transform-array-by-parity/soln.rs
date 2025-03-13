impl Solution {
    pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = nums.iter().map(|x| x&1).collect::<Vec<_>>();
        ans.sort();
        ans
    }
}
