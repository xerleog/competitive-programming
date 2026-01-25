impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        nums.windows(k as usize).map(|x| x[(k-1) as usize]-x[0]).min().unwrap()
    }
}
