impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        (((n+1) as usize).next_power_of_two()-1) as i32
    }
}
