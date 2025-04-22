impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        (1..height.len()).filter_map(|x| (height[x-1]>threshold).then_some( x as i32)).collect::<Vec<_>>()   
    }
}
