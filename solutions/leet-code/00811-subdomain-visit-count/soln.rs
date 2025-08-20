use std::collections::BTreeMap;
impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut ans :BTreeMap<_,i32>= BTreeMap::new();
        let mut sol = vec![];
        for i in cpdomains
        {
            let val = i.split_whitespace().collect::<Vec<_>>();
            let n = val[0].parse::<i32>().unwrap();
            let temp = val[1].split(".").collect::<Vec<_>>();
            for j in 0..temp.len()
            {
                let domain = temp[j..].join(".");
                *ans.entry(domain).or_default()+=n;
            }
        }
        for (i,j) in ans
        {
            sol.push(format!("{} {}",j,i));
        }
        sol
    }
}
