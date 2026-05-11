impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter().flat_map(|x| x.to_string().as_bytes().into_iter().map(|y| (y-48) as i32).collect::<Vec<_>>()).collect()
    }
}
