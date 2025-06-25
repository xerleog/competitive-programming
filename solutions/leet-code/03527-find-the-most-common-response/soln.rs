use std::collections::{BTreeMap,BTreeSet};
impl Solution {
    pub fn find_common_response(responses: Vec<Vec<String>>) -> String {
        let mut ans : BTreeMap<String,i32>= BTreeMap::new();
        for i in responses
        {
            let mut temp = BTreeSet::from_iter(i.into_iter());
            temp.into_iter().for_each(|x| *ans.entry(x).or_default()+=1);
        }
        let ma = ans.clone().into_values().max().unwrap();
        for (i,j) in ans
        {
            if j==ma
            {   return i;}
        }
        "".to_string()
    }
}
