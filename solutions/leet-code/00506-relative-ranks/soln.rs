use std::collections::BTreeMap;
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut ans = score.clone();
        ans.sort_by(|a,b| b.cmp(&a));
        let val = BTreeMap::from_iter(ans.into_iter().enumerate().map(|(a,b)| (b,a+1)));
        let mut sol = vec![];
        for i in score.iter()
        {
            if val[i]==1
            {
                sol.push("Gold Medal".to_string());
            }
            else if val[i]==2
            {
                sol.push("Silver Medal".to_string());
            }
            else if val[i]==3
            {
                sol.push("Bronze Medal".to_string());
            }else if val[i]>3
            {
                sol.push(format!("{}",val[i]));
            }
        } 
        sol        
    }
}
