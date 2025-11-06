impl Solution {
    pub fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
        tasks.into_iter().map(|x| x[0]+x[1]).min().unwrap()
    }
}
