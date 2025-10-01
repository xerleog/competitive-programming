impl Solution {
    pub fn num_water_bottles(a: i32, b: i32) -> i32 {
        a+(a-1)/(b-1)
    }
}
