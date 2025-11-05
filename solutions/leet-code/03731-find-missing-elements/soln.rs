impl Solution {
    pub fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        nums.windows(2).filter(|x| x[0]+1!=x[1]).flat_map(|y| (y[0]+1..y[1])).collect::<Vec<_>>()
    }
}
