use std::collections::{BTreeMap,BTreeSet};
impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut ans :BTreeMap<i32,BTreeSet<i32>> = BTreeMap::new();
        logs.into_iter().for_each(|x| { ans.entry(x[0]).or_insert_with(BTreeSet::new).insert(x[1]);});
        let mut sol = vec![0;k as usize];
        ans.into_values().for_each(|x| if x.len()>0 { sol[x.len()-1]+=1;});
        sol
    }
}
