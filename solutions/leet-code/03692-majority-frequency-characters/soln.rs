use std::collections::BTreeMap;
impl Solution {
    pub fn majority_frequency_group(s: String) -> String {
        let mut ans = vec![0;26];
        s.clone().as_bytes().into_iter().for_each(|&x| ans[(x as usize)-97]+=1);
        let mut sol : BTreeMap<_,i32>= BTreeMap::new();
        for i in ans.clone()
        {
            if i!=0 { *sol.entry(i).or_default()+=1;}
        }
        let max_key = sol.iter().max_by_key(|&(_k, v)| v).map(|(&k, _v)| k).unwrap();
        let mut val = "".to_string();
        for (i,j) in ans.into_iter().enumerate()
        {
            if j==max_key
            {
                val.push((i as u8+97) as char);
            }
        }
        val
    }
}
