impl Solution {
    pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        nums.chunks(2).map(|x| vec![x[1],x[0]]).flatten().collect::<Vec<_>>()
    }
}
