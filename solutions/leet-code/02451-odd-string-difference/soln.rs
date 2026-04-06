use std::collections::BTreeMap;
impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut ans : BTreeMap<Vec<_>,Vec<_>> = BTreeMap::new();
        let mut word = words.clone().into_iter().map(|x| x.as_bytes().to_vec()).collect::<Vec<_>>();
        for i in 0..words.len()
        {
            let val = word[i].windows(2).map(|x| x[1]-x[0]).collect::<Vec<_>>();
            ans.entry(val).or_insert_with(Vec::new).push(i);
        }
        ans.iter().find(|(_, v)| v.len() == 1).map(|(_, v)| &words[v[0]]).unwrap().to_string()
    }
}
