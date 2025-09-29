struct P {
    v: Vec<i32>,
    memo: Vec<Vec<i32>>,
}

impl P {
    fn dfs(&mut self, i: usize, j: usize) -> i32 {
        if j - i < 2 { return 0; }
        if self.memo[i][j] != 0 { return self.memo[i][j]; }
        let res = (i+1..j)
            .map(|k| self.dfs(i,k) + self.dfs(k,j) + self.v[i]*self.v[k]*self.v[j])
            .min()
            .unwrap();
        self.memo[i][j] = res;
        res
    }
}

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut p = P { v: values, memo: vec![vec![0; n]; n] };
        p.dfs(0, n-1)
    }
}
