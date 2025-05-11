impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        arr.windows(3).any(|x| x[0]%2==1 && x[1]%2==1 &&x[2]%2==1 )
    }
}
