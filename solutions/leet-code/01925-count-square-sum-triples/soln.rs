use std::collections::BTreeSet;
impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut ans =BTreeSet::new();
        for i in 1..=n
        {
            for j in 1..=n
            {
                for k in 1..=n
                {
                    if i!=j && j!=k && k!=i && i*i+j*j==k*k
                    {   ans.insert((i,j,k));}
                }
            }
        }
        ans.len() as i32
    }
}
