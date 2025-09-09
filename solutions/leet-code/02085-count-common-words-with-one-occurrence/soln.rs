use std::collections::BTreeMap;
impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut feq1:BTreeMap<String,i32> = BTreeMap::new();
        let mut feq2:BTreeMap<String,i32> = BTreeMap::new();
        words1.into_iter().for_each(|x|  *feq1.entry(x).or_default() += 1);
        words2.into_iter().for_each(|x|  *feq2.entry(x).or_default() += 1);
        feq1.iter().filter(|&(i, &j)| j == 1 && feq2.get(i).copied() == Some(1)).count() as i32
    }
}
