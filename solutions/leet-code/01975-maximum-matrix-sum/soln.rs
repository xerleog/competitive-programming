impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut ans:Vec<i32> = matrix.into_iter().flatten().collect();
        ans.sort_by(|a,b| a.abs().cmp(&b.abs()));
        let n = ans.clone().into_iter().filter(|&x| x<0).count();
        println!("{:?}",ans);
        if n%2==0
        {   ans.into_iter().map(|x| x.abs() as i64).sum::<i64>()}
        else
        {   ans[1..].into_iter().map(|x| x.abs() as i64).sum::<i64>()-(ans[0].abs() as i64)}
    }
}
