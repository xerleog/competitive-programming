impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        nums.into_iter().enumerate().map(|(x,y)| if y==target { ((x as i32) - start).abs()} else { i32::MAX}).min().unwrap()
    }
}
