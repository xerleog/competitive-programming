impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut n = n as u64;
        let mut ans = vec![1_u64];
        (0..=32).for_each(|x| ans.push(ans[x]*2));
        let mut temp = vec![];
        for i in ans.clone().into_iter().rev()
        {
            if i<=n
            {
                n-=i;
                temp.insert(0,i);
            }
        }
        let mut sol = vec![];
        for i in queries
        {
            sol.push((i[0]..=i[1]).fold(1,|a,c| (a*temp[c as usize])%1_000_000_007)as i32);
        }
        sol
    }
}
