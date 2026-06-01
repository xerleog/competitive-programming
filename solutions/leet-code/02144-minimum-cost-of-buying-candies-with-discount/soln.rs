impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort();
        cost.rchunks(3).flat_map(|c| &c[c.len() / 3..]).sum()
    }
}
