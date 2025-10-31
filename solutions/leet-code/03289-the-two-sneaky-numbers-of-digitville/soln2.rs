impl Solution {
    pub fn get_sneaky_numbers(mut nums: Vec<i32>) -> Vec<i32> {
           nums.sort();
           nums.windows(2).filter(|x| x[0]==x[1]).map(|x| x[0]).collect::<Vec<_>>()
    }
}
