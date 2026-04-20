impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let mut ans = vec![vec![];101];
        let mut ma = 0;
        colors.into_iter().enumerate().for_each(|(x,y)| ans[y as usize].push(x as i32));
        for i in 0..100
        {
            for j in i+1..101
            {
                if ans[i].len()>0 && ans[j].len()>0
                {
                    let temp1 = ans[j].last().unwrap()-ans[i][0];
                    let temp2 = ans[i].last().unwrap()-ans[j][0];
                    ma = ma.max(temp1).max(temp2);
                }
            }
        }
        ma
    }
}
