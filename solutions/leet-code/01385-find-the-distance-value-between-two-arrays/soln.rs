impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
    arr1.into_iter()
    .filter(|&a| arr2.clone().into_iter().all(|b| (b - a).abs() > d))
    .count() as i32
    }
}
