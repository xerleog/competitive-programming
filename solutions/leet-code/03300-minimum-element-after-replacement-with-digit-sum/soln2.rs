impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.into_iter().map(|x| x.to_string().as_bytes().into_iter().map(|y| (y-48) as i32).sum::<i32>()).min().unwrap()
    }
}
