impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len();
        let mut ans = vec![0;n+2];
        edges.into_iter().for_each(|x| {ans[x[0] as usize]+=1; ans[x[1]as usize]+=1;});
        ans.into_iter().position(|x| x==n).unwrap() as i32
    }
}
