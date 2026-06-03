impl Solution {
    pub fn minimum_swaps(nums: Vec<i32>) -> i32 {
        let ans = nums.clone().into_iter().enumerate().filter(|x| x.1==0).collect::<Vec<_>>();
        ans.clone().into_iter().filter(|x| x.0< nums.len()-ans.len()).count() as i32

    }
}
