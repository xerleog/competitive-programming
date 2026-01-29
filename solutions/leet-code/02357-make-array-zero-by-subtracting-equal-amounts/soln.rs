impl Solution {
    pub fn minimum_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums.dedup();
        nums.into_iter().filter(|&x| x!=0).count() as i32
    }
}
