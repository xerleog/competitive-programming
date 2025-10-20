impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations.into_iter().fold(0, |a,c| if c.contains(&"-") { a-1} else { a+1})
    }
}
