use std::collections::BTreeMap;
impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans :BTreeMap<i32,Vec<Vec<i32>>> = BTreeMap::new();
        arr.sort();
        for i in arr.windows(2)
        {
            let diff = (i[1]-i[0]);
            ans.entry(diff).or_insert_with(Vec::new).push(vec![i[0],i[1]]);

        }
        let (_,mut sol) = ans.pop_first().unwrap();
        sol
    }

}
