impl Solution {
    pub fn similar_pairs(mut words: Vec<String>) -> i32 {
        let mut ans = words.into_iter().map(|x| x.as_bytes().to_vec()).collect::<Vec<_>>();
        (0..ans.len()).for_each(|x| ans[x].sort());
        (0..ans.len()).for_each(|x| ans[x].dedup());
        let mut sol = 0;
        for i in 0..ans.len()-1
        {
            for j in i+1..ans.len()
            {
                if ans[i]==ans[j]
                {   sol+=1;}
            }
        }
        sol
    }
}
