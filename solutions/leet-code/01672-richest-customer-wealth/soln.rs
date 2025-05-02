impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.into_iter().map(|x| x.into_iter().sum::<i32>()).max().unwrap()
    }
}
