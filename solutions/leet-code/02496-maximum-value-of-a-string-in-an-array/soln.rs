impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.iter()
            .map(|c| match c.parse::<i32>() {
                Ok(v) => v,
                Err(_) => c.len() as i32,
            })
            .max()
            .unwrap()
    }
}
