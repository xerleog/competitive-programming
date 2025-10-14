impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        nums.windows((2*k) as usize).map(|x| x[0..k as usize].windows(2).all(|j| j[0]<j[1]) && x[k as usize..].windows(2).all(|j| j[0]<j[1])).fold(false, |a,c| a|c)
    }
}
