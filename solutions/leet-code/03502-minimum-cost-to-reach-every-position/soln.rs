impl Solution {
    pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0;cost.len()];
        let mut val = i32::MAX;
        (0..cost.len()).into_iter().for_each(|x| if cost[x] < val { val = cost[x]; ans[x] = val;} else { ans[x] = val} );
        ans   
    }
}
