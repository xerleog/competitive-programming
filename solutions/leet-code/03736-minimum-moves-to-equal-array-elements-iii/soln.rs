impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let ma = nums.clone().into_iter().max().unwrap();
        nums.into_iter().map(|x| ma-x).sum::<i32>()
    }
}
