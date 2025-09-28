impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums.windows(3).map(|x| if x[2]<x[1]+x[0] { x.into_iter().sum::<i32>()} else {0}).max().unwrap()
    }
}
