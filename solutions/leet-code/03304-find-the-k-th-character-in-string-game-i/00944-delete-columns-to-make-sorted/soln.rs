impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut ans = strs.into_iter().map(|x| x.as_bytes().to_vec()).collect::<Vec<_>>();
        let mut sol = 0;
        for j in 0..ans[0].len()
        {
            let mut val = true;
            for i in 0..ans.len()-1
            {
                if ans[i][j]>ans[i+1][j]
                {
                    val = false;
                    break;
                }
            }
            if !val
            {   sol+=1;}
        }
        sol
    }
}
