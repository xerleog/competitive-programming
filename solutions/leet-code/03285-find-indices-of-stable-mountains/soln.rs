impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        (1..height.len() as i32).filter(|x| height[(x-1)as usize]>threshold).collect::<Vec<_>>()   
    }
}
