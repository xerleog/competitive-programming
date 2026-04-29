impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
       (1..).map(|x| x*k).find(|y| !nums.contains(y)).unwrap() 
    }
}
