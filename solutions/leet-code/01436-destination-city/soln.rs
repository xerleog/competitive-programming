impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut ans = vec![paths[0].clone()];
        while ans.len()!=paths.len()
        {
            for i in paths.clone().into_iter()
            {
                if i[1]==ans[0][0]
                {
                    ans.insert(0,i);
                }
                else if ans.last().unwrap()[1]== i[0]
                {
                    ans.push(i);
                }
            }
        }
        ans.last().unwrap()[1].clone()
    }
}
