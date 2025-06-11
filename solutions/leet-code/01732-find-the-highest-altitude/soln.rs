impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.into_iter().scan(0,|a,c| {*a+=c; Some(*a)}).max().unwrap().max(0)
    }
}
