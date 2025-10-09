impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        arr.windows(3).all(|x| 2*x[1]==x[0]+x[2])
    }
}
