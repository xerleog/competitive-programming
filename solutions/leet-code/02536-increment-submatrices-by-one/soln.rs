impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut ans = vec![vec![0;n];n];
        for i in queries
        {
            for p in i[0]..=i[2]
            {
                for q in i[1]..=i[3]
                {
                    ans[p as usize][q as usize]+=1;
                }
            }
        }
        ans
    }
}
